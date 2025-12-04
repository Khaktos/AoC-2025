use std::fs;
pub fn read_and_splitline(fname: String) -> Vec<String> {
    let contents = fs::read_to_string(fname).expect("I messed up");
    let lines: Vec<String> = contents
        .split('\n')
        .filter(|e| e.len() != 0)
        .map(|e| e.to_string())
        .collect();
    lines
}
pub fn read_and_split_at(fname: String, delim: char) -> Vec<String> {
    let contents = fs::read_to_string(fname).expect("I messed up");
    let lines: Vec<String> = contents
        .split(delim)
        .filter(|e| e.len() != 0)
        .map(|e| e.trim())
        .map(|e| e.to_string())
        .collect();
    lines
}
pub fn read_matrix(fname: String) -> Vec<Vec<String>> {
    let contents = fs::read_to_string(fname).expect("I messed up");
    let lines: Vec<Vec<String>> = contents
        .split('\n')
        .filter(|e| e.len() != 0)
        .map(|e| e.trim().chars().map(|c| c.to_string()).collect())
        .collect();
    lines
}
