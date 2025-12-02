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
