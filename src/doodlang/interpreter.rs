use std::collections::HashMap;
use std::ops::Index;
use crate::doodlang::parser::{FunctionManager, generate_tree, smart_generate_tree};
use crate::eq_solver::approx::{SecantSolver, QuadFind};

fn process(code: &str) -> Vec<String> {
    let processed = code.replace(" ", "");
    let mut recordings: Vec<String> = vec![];
    let mut scopes: Vec<String> = vec![];
    for i in 0..processed.len() {
        let current_char = &processed[i..i + 1];
        for scope in &mut recordings {
            *scope += current_char;
        }
        if current_char == "{" {
            recordings.push("".to_string());
        }
        if current_char == "}" {
            scopes.push(recordings.pop().unwrap());
        }
    }
    return scopes;
}

pub struct DoodlangInterpreter<'a> {
    code: &'a str,
    solver: SecantSolver,
    quad_solver: QuadFind,
    solve_key: &'a str,
}

impl DoodlangInterpreter<'_> {
    pub fn new(code: &str) -> DoodlangInterpreter {
        //ln(x + 1)
        //ln_1p(x) = ln(x + 1)
        DoodlangInterpreter {
            code,
            solver: SecantSolver::new(
                [-5f64, 5f64],
                0.1f64,
                100,
                10000,
                0f64,
            ),
            quad_solver: QuadFind::new(
                [-5f64, 5f64],
                3f64,
                100,
                10000,
                0f64
            ),
            solve_key: "solve",
        }
    }
    pub fn run(&mut self) {
        let scopes: Vec<String> = process(self.code);
        for scope in scopes {
            self.run_scope(Box::new(scope));
        }
    }

    fn run_scope(&mut self, scope: Box<String>) {
        let lines = self.process_scope(scope);
        let mut variables: Vec<(&str, f64)> = Vec::new();
        for line in &lines {
            self.execute_line(line, &mut variables);
        }
    }

    fn process_scope(&mut self, scope: Box<String>) -> Vec<Box<String>> {
        let mut lines: Vec<Box<String>> = Vec::new();
        let mut record = "".to_string();
        for i in 0..scope.len() {
            let cur = &scope[i..i + 1];
            if cur == ";" {
                lines.push(Box::new(record.clone()));
                record = "".to_string();
            } else {
                record += cur;
            }
        }
        return lines;
    }

    fn execute_line<'a>(&mut self, line: &'a Box<String>, variables: &mut Vec<(&'a str, f64)>) {
        if line.contains(":") {
            let end_index = line.find(":").unwrap();
            let mut start_index = 0;
            for i in 0..end_index {
                if line[i..i + 1].chars().all(char::is_alphanumeric) {
                    start_index = i;
                    break;
                }
            }
            let var_name = &line[start_index..end_index];
            let value = match line[end_index + 1..line.len()].parse::<f64>() {
                Ok(x) => x,
                Err(_) => {
                    println!("This value cannot be parsed in line: {}", &line[end_index + 1..line.len()]);
                    unimplemented!();
                }
            };
            variables.push((var_name, value));
        } else if line.contains(self.solve_key) {
            let end_index = line.find(self.solve_key).unwrap() + self.solve_key.len();
            let index = end_index + self.parse_variable(&line[end_index..line.len()]);
            if &line[index..index + 2] != "<-" {
                println!("You need to put an arrow: {} to signify that you have finished the solve definition", "<-");
                unimplemented!();
            }
            let variable = &line[end_index + 1..index - 1];
            let mut function_manager = Box::new(FunctionManager::new(variables));
            let formula = &line[index + 2..line.len()];
            println!("formula: {}", formula);
            let tree = smart_generate_tree(&formula.to_string());
            let mut function = function_manager.generate_func(&formula, &tree);
            let solution = self.solver.solve_for_param(&mut function, &mut function_manager, variable);
            function_manager.variables[*function_manager.ids.get(variable).unwrap()] = solution;
            println!("{}: {} with abs err: {}", variable, solution, function.output(&function_manager).abs());
        }
    }

    fn parse_variable(&mut self, line: &str) -> usize {
        if &line[0..1] != "(" {
            println!("Parentheses not found: {}", &line[0..1]);
            unimplemented!();
        }
        let mut index = 1usize;
        loop {
            let current_char = &line[index..index + 1];
            index += 1;
            if current_char == ")" {
                break;
            }
        }
        return index;
    }
}