use std::fs;
pub fn read_and_splitline(fname: String) -> Vec<String> {
    let contents = fs::read_to_string(fname).expect("I messed up");
    let lines: Vec<String> = contents
        .lines()
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
        .lines()
        .filter(|e| e.len() != 0)
        .map(|e| e.trim().chars().map(|c| c.to_string()).collect())
        .collect();
    lines
}
pub fn read_matrix_delim(fname: String, delim: char) -> Vec<Vec<String>> {
    let contents = fs::read_to_string(fname).expect("I messed up");
    let lines: Vec<Vec<String>> = contents
        .lines()
        .filter(|e| e.len() != 0)
        .map(|e| {
            e.trim()
                .split(delim)
                .filter(|s| s.len() != 0)
                .map(|s| s.to_string())
                .collect()
        })
        .collect();
    lines
}
pub fn read_line_groups(fname: String, separator: &str) -> (Vec<String>, Vec<String>) {
    let contents = fs::read_to_string(fname).expect("I messed up");
    let mut lines: Vec<String> = contents.lines().map(|e| e.to_string()).collect();
    let mut rest = lines.split_off(
        lines
            .iter()
            .position(|e| *e == separator.to_string())
            .unwrap(),
    );
    lines = lines.into_iter().filter(|e| e.len() != 0).collect();
    rest = rest.into_iter().filter(|e| e.len() != 0).collect();
    (lines, rest)
}
