mod day6;
use day6::{solve_a, solve_b};
mod utils;
use std::env;
use utils::read_and_split_at;
use utils::read_and_splitline;
use utils::read_line_groups;
use utils::read_matrix;
use utils::read_matrix_delim;
fn main() {
    let args: Vec<String> = env::args().collect();
    // let matrix = read_matrix(args[1].clone());
    let matrix = read_matrix_delim(args[1].clone(), ' ');
    let lines = read_and_splitline(args[1].clone());
    // let lines = read_line_groups(args[1].clone(), "");
    // let lines = read_and_split_at(args[1].clone(), ',');
    println!("Solution A: {}", solve_a(&matrix));
    // println!("Solution B: {}", solve_b(&matrix));
    // println!("Solution A: {}", solve_a(&lines));
    println!("Solution B: {}", solve_b(&lines));
}
