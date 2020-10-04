use std::fmt::Display;
use crate::eq_solver::matrix::Matrix;
use doodlang::parser::{Parser, FunctionManager, Node, generate_tree};
use crate::eq_solver::approx::{PolyApprox, SecantSolver};
use crate::doodlang::interpreter::DoodlangInterpreter;
use std::fs;
use text_io::read;

mod eq_solver;
mod doodlang;

fn main() {
    let file: String = read!();
    let code = fs::read_to_string(file)
        .expect("Something went wrong reading the file");
    let mut interpreter = DoodlangInterpreter::new(&code[0..code.len()]);
    interpreter.run();
}

pub fn print_tree(node: &Node<[usize; 2]>, formula: &String) {
    if node.children.is_empty() {
        print!("({})", &formula[node.value[0]..node.value[1]]);
    } else {
        for child in &node.children {
            print!("(");
            print_tree(child, formula);
            print!(")");
        }
    }
}

pub fn print_vec<T: Display>(vec: &Vec<T>) {
    for val in vec {
        print!("{} ", val);
    }
}

fn stuff(range: usize) -> [usize; 2] {
    let mut vec: Vec<[usize; 2]> = Vec::new();
    for i in 0..range {
        vec.push([0usize, i]);
    }
    return vec[0];
}
