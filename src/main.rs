mod day2;
use day2::{solve_a, solve_b};
mod utils;
use std::env;
use utils::read_and_split_at;
use utils::read_and_splitline;
fn main() {
    let args: Vec<String> = env::args().collect();
    // let lines = read_and_splitline(args[1].clone());
    let lines = read_and_split_at(args[1].clone(), ',');
    println!("Solution A: {}", solve_a(&lines));
    println!("Solution B: {}", solve_b(&lines));
}
