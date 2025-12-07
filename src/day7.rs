pub fn solve_b(lines: &Vec<String>) -> u64 {
    let mut state: Vec<u64> = lines[0]
        .chars()
        .map(|c| if c == 'S' { 1 } else { 0 })
        .collect();
    let mut new_state: Vec<u64> = vec![0; state.len()];

    let splitters: Vec<Vec<bool>> = lines[2..]
        .iter()
        .step_by(2)
        .map(|line| line.chars().map(|c| c == '^').collect())
        .collect();

    for splitter in splitters.iter() {
        for (idx, _) in state.iter().enumerate() {
            let left = if idx as i32 - 1 >= 0 && (splitter[idx - 1] && state[idx - 1] > 0) {
                state[idx - 1]
            } else {
                0
            };
            let right = if idx + 1 < state.len() && (splitter[idx + 1] && state[idx + 1] > 0) {
                state[idx + 1]
            } else {
                0
            };
            let up = if state[idx] > 0 && !splitter[idx] {
                state[idx]
            } else {
                0
            };
            new_state[idx] = left + right + up;
        }
        state = new_state.clone();
    }
    new_state.iter().sum::<u64>()
}
pub fn solve_a(lines: &Vec<String>) -> i32 {
    let mut state: Vec<bool> = lines[0].chars().map(|c| c == 'S').collect();
    let mut new_state: Vec<bool> = vec![false; state.len()];

    let splitters: Vec<Vec<bool>> = lines[2..]
        .iter()
        .step_by(2)
        .map(|line| line.chars().map(|c| c == '^').collect())
        .collect();

    let mut splits = 0;
    for splitter in splitters.iter() {
        for (idx, _) in state.iter().enumerate() {
            let left = if idx as i32 - 1 >= 0 {
                splitter[idx - 1] && state[idx - 1]
            } else {
                false
            };
            let right = if idx as i32 + 1 < state.len() as i32 {
                splitter[idx + 1] && state[idx + 1]
            } else {
                false
            };
            let up = state[idx] && !splitter[idx];
            new_state[idx] = left || right || up;
            splits += left as i32 + right as i32;
        }
        state = new_state.clone();
    }
    splits / 2
}
