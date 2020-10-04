use std::collections::HashMap;
use crate::eq_solver::function::{Function, Sum, Mul, Variable, Sub, Div, Pow, Ln, Log, Cosine, Sine, Constant};
use std::ops::Deref;
use crate::print_vec;
use std::rc::Rc;
use std::cmp::min;

pub struct Parser;

const OPER: [&str; 6] = ["=", "-", "+", "/", "*", "^"];
const PRIORITIES: [usize; 6] = [0, 1, 1, 2, 2, 3];
const FUNC: [&str; 4] = ["log", "ln", "sin", "cos"];

pub fn smart_generate_tree(formula: &str) -> Node<[usize; 2]> {
    let mut cur_scope = 0usize;
    let mut scopes = Vec::new();
    for i in 0..formula.len() {
        let cur = &formula[i..i + 1];
        if cur == "(" {
            cur_scope += 1;
            scopes.push(cur_scope);
        } else if cur == ")" {
            scopes.push(cur_scope);
            cur_scope -= 1;
        } else {
            scopes.push(cur_scope);
        }
    }
    let mut parent = Node::new([0, formula.len()]);
    recursive_generate(0, &mut parent, formula, &scopes);
    return parent;
}

pub fn recursive_generate(reference_point: usize, parent: &mut Node<[usize; 2]>, formula: &str, scopes: &Vec<usize>) {
    let current_scope = scopes[0];
    let min_local_scope = find_min_local_scope(formula);
    let mut max_index = 0;
    let mut min_priority = usize::MAX;
    let mut found_oper = false;
    for i in 0..OPER.len() {
        let oper = OPER[i];
        let pos = operator_in_scope(formula, oper, min_local_scope);
        if pos.is_some() {
            let index = pos.unwrap();
            if PRIORITIES[i] > min_priority {
                break;
            } else {
                if index > max_index {
                    max_index = index;
                    min_priority = PRIORITIES[i];
                    found_oper = true;
                }
            }
        }
    }
    if found_oper {
        let index = max_index;
        let mut end_index = formula.len();
        let mut local_scope = 0;
        for i in index + 1..formula.len() {
            let cur = &formula[i..i + 1];
            if cur == "(" {
                local_scope += 1;
            } else if cur == ")" {
                local_scope -= 1;
            }
            if local_scope < 0 {
                end_index = i;
                break;
            }
        }
        let mut start_index = 0;
        local_scope = 0;
        for i in (0..index).rev() {
            let cur = &formula[i..i + 1];
            if cur == ")" {
                local_scope += 1;
            } else if cur == "(" {
                local_scope -= 1;
            }
            if local_scope < 0 {
                start_index = i + 1;
                break;
            }
        }
        let mut left = Node::new([start_index + reference_point, index + reference_point]);
        let mut right = Node::new([index + reference_point + 1, end_index + reference_point]);
        recursive_generate(
            reference_point + start_index,
            &mut left,
            &formula[start_index..index],
            &scopes[start_index..index].to_vec(),
        );
        recursive_generate(
            reference_point + index + 1,
            &mut right,
            &formula[index + 1..end_index],
            &scopes[index + 1..end_index].to_vec(),
        );
        parent.children.push(left);
        parent.children.push(right);
        return;
    }
    for func in &FUNC {
        if formula.len() > func.len() + 2 {
            let pos = function_in_scope(formula, func, min_local_scope);
            if pos.is_some() {
                let func_start = pos.unwrap();
                let start = func_start + func.len();
                let start_scope = scopes[start];
                let mut end_index = formula.len();
                for i in start..formula.len() {
                    if scopes[i] != start_scope {
                        end_index = i;
                    }
                }
                parse_csv(
                    reference_point + start + 1,
                    parent,
                    &formula[start + 1..end_index - 1],
                    &scopes[start + 1..end_index - 1].to_vec(),
                );
                return;
            }
        }
    }
    let mut end_parse = true;
    for i in 0..scopes.len() {
        let cur = &formula[i..i + 1];
        if cur == "(" || cur == ")" || current_scope < scopes[i] {
            end_parse = false;
            break;
        }
    }
    if end_parse {
        return;
    }
    let start = parent.value[0] + 1;
    let end = parent.value[1] - 1;
    if end > start {
        let mut sub_node = Node::new([start, end]);
        recursive_generate(reference_point + 1, &mut sub_node, &formula[1..formula.len() - 1], &scopes[1..formula.len() - 1].to_vec());
        parent.children.push(sub_node);
    }
}

fn parse_csv(reference_point: usize, node: &mut Node<[usize; 2]>, values: &str, scopes: &Vec<usize>) {
    let mut act_pos = 0;
    loop {
        let comma = (&values[act_pos..values.len()]).find(",");
        if comma.is_none() {
            let mut inner = Node::new([reference_point + act_pos, reference_point + values.len()]);
            recursive_generate(
                reference_point + act_pos,
                &mut inner,
                &values[act_pos..values.len()],
                &scopes[act_pos..values.len()].to_vec(),
            );
            node.children.push(inner);
            break;
        } else {
            let pos = comma.unwrap();
            let mut inner = Node::new([reference_point + act_pos, reference_point + act_pos + pos]);
            recursive_generate(
                reference_point + act_pos,
                &mut inner,
                &values[act_pos..act_pos + pos],
                &scopes[act_pos..act_pos + pos].to_vec(),
            );
            act_pos += pos + 1;
            node.children.push(inner);
        }
    }
}

fn operator_in_scope(formula: &str, operator: &str, current_scope: usize) -> Option<usize> {
    let mut scope = 0;
    for i in (0..formula.len()).rev() {
        let cur = &formula[i..i + 1];
        if cur == ")" {
            scope += 1;
        } else if cur == "(" {
            scope -= 1;
        }
        if scope == current_scope && &formula[i..i + 1] == operator {
            return Option::Some(i);
        }
    }
    return Option::None;
}

fn function_in_scope(formula: &str, function: &str, current_scope: usize) -> Option<usize> {
    if formula.len() < function.len() {
        return Option::None;
    }
    let func_len = function.len();
    let mut scope = 0;
    for i in 0..formula.len() + 1 - func_len {
        let cur = &formula[i..i + 1];
        if cur == "(" {
            scope += 1;
        } else if cur == ")" {
            scope -= 1;
        }
        if scope == current_scope && &formula[i..i + func_len] == function {
            return Option::Some(i);
        }
    }
    return Option::None;
}

pub fn generate_tree(formula: &str) -> Node<[usize; 2]> {
    let mut scope = 0usize;
    let mut record_positions: Vec<usize> = Vec::new();
    let range = [0, formula.len()];
    let root_node: Node<[usize; 2]> = Node::new(range);
    let mut nodes: Vec<Node<[usize; 2]>> = vec![root_node];
    let mut scopes: Vec<usize> = Vec::new();
    for i in 0..formula.len() {
        let cur = &formula[i..i + 1];
        if cur == "(" {
            scope += 1;
            record_positions.push(i);
            nodes.push(Node::new([0, 0]));
        } else if cur == ")" {
            scope -= 1;
            let mut node = nodes.pop().unwrap();
            node.set_value([record_positions.pop().unwrap(), i + 1]);
            let last_index = nodes.len() - 1;
            nodes[last_index].children.push(node);
            scopes.insert(0, scope);
        }
    }
    return nodes.pop().unwrap();
}

pub struct Node<T> {
    pub value: T,
    pub children: Vec<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            children: Vec::new(),
        }
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }
}

pub struct FunctionManager<'a> {
    pub ids: HashMap<&'a str, usize>,
    pub variables: Vec<f64>,
}

impl FunctionManager<'_> {
    pub fn new<'a>(variables: &'a Vec<(&str, f64)>) -> FunctionManager<'a> {
        let mut ids = HashMap::new();
        let mut values = Vec::new();
        for i in 0..variables.len() {
            ids.insert(variables[i].0, i);
            values.push(variables[i].1);
        }
        FunctionManager {
            ids,
            variables: values,
        }
    }
    pub fn generate_func(&mut self, formula: &str, node: &Node<[usize; 2]>) -> Box<dyn Function> {
        return self.recursive_parse(formula, node);
    }


    fn recursive_parse(&mut self, formula: &str, node: &Node<[usize; 2]>) -> Box<dyn Function> {
        let start_index = node.value[0];
        let end_index = node.value[1];
        let node_formula = &formula[start_index..end_index];
        let min_local_scope = find_min_local_scope(node_formula);
        if node.children.len() == 1 {
            let mut min_func_scope = function_in_scope(node_formula, "ln", min_local_scope);
            if min_func_scope.is_some() {
                return Box::new(
                    Ln {
                        input: self.recursive_parse(formula, &node.children[0]),
                    }
                );
            }
            let mut min_func_scope = function_in_scope(node_formula, "sin", min_local_scope);
            if min_func_scope.is_some() {
                return Box::new(
                    Sine {
                        input: self.recursive_parse(formula, &node.children[0]),
                    }
                );
            }
            let mut min_func_scope = function_in_scope(node_formula, "cos", min_local_scope);
            if min_func_scope.is_some() {
                return Box::new(
                    Cosine {
                        input: self.recursive_parse(formula, &node.children[0]),
                    }
                );
            }
            return self.recursive_parse(formula, &node.children[0]);
        }
        if find_index(node_formula, "-", min_local_scope).is_some() ||
            find_index(node_formula, "=", min_local_scope).is_some() {
            return Box::new(
                Sub {
                    left: self.recursive_parse(formula, &node.children[0]),
                    right: self.recursive_parse(formula, &node.children[1]),
                }
            );
        } else if find_index(node_formula, "+", min_local_scope).is_some() {
            return Box::new(
                Sum {
                    left: self.recursive_parse(formula, &node.children[0]),
                    right: self.recursive_parse(formula, &node.children[1]),
                }
            );
        } else if find_index(node_formula, "/", min_local_scope).is_some() {
            return Box::new(
                Div {
                    left: self.recursive_parse(formula, &node.children[0]),
                    right: self.recursive_parse(formula, &node.children[1]),
                }
            );
        } else if find_index(node_formula, "*", min_local_scope).is_some() {
            return Box::new(
                Mul {
                    left: self.recursive_parse(formula, &node.children[0]),
                    right: self.recursive_parse(formula, &node.children[1]),
                }
            );
        } else if find_index(node_formula, "^", min_local_scope).is_some() {
            return Box::new(
                Pow {
                    base: self.recursive_parse(formula, &node.children[0]),
                    power: self.recursive_parse(formula, &node.children[1]),
                }
            );
        } else {
            let min_func_scope = function_in_scope(node_formula, "log", min_local_scope);
            if min_func_scope.is_some() {
                return Box::new(
                    Log {
                        base: self.recursive_parse(formula, &node.children[0]),
                        input: self.recursive_parse(formula, &node.children[1]),
                    }
                );
            }
            let val = self.ids.get(node_formula);
            if val.is_some() {
                Box::new(
                    Variable {
                        id: *self.ids.get(node_formula).unwrap(),
                    }
                )
            } else {
                Box::new(
                    Constant {
                        val: node_formula.parse::<f64>().unwrap(),
                    }
                )
            }
        }
    }
}

fn find_min_local_scope(formula: &str) -> usize {
    let mut scope: usize = 0;
    let mut min_scope: usize = usize::MAX;
    let mut last = true;
    for i in 0..formula.len() {
        let cur = &formula[i..i + 1];
        if last == false {
            min_scope = min(scope, min_scope);
        }
        if cur == "(" {
            scope += 1;
            last = true;
        } else {
            last = false;
            if cur == ")" {
                scope -= 1;
            }
        }
    }
    return min_scope;
}

fn find_index(formula: &str, operation: &str, local_scope: usize) -> Option<usize> {
    let mut scope: usize = 0;
    for i in 0..formula.len() {
        let cur = &formula[i..i + 1];
        if cur == "(" {
            scope += 1;
        } else if cur == ")" {
            scope -= 1;
        }
        if cur == operation && scope == local_scope {
            return Option::Some(i);
        }
    }
    return Option::None;
}