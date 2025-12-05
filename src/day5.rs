fn parse_contents(contents: &(Vec<String>, Vec<String>)) -> (Vec<Vec<u64>>, Vec<u64>) {
    let (interv_str, elem_str) = contents;
    let mut intervals: Vec<Vec<u64>> = interv_str
        .iter()
        .map(|int| {
            int.split('-')
                .map(|e| e.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    intervals.sort();
    let elements: Vec<u64> = elem_str.iter().map(|e| e.parse::<u64>().unwrap()).collect();
    (intervals, elements)
}
fn merge_intervals(intervals: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut stack: Vec<u64> = vec![intervals[0][0], intervals[0][1]];
    for int in intervals.iter() {
        let peek = stack[stack.len() - 1];
        if peek > int[0] - 1 {
            match stack.pop_if(|e| *e < int[1]) {
                Some(_) => stack.push(int[1]),
                None => continue,
            };
        } else {
            stack.push(int[0]);
            stack.push(int[1]);
        }
    }
    let mut merged = vec![];
    for idx in (0..stack.len()).step_by(2) {
        merged.push(vec![stack[idx], stack[idx + 1]]);
    }
    merged
}
pub fn solve_b(contents: &(Vec<String>, Vec<String>)) -> u64 {
    let (intervals, _) = parse_contents(contents);
    let merged = merge_intervals(&intervals);
    let mut sum = 0;
    for int in merged.iter() {
        sum += int[1] - int[0] + 1;
    }
    sum
}
pub fn solve_a(contents: &(Vec<String>, Vec<String>)) -> i32 {
    let (intervals, elements) = parse_contents(contents);
    let merged = merge_intervals(&intervals);
    let mut sum = 0;
    for elem in elements.iter() {
        for int in merged.iter() {
            if *elem >= int[0] && *elem <= int[1] {
                sum += 1;
            }
        }
    }
    sum
}
