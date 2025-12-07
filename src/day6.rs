pub fn solve_b(lines: &Vec<String>) -> u64 {
    let opers: Vec<&str> = lines[lines.len() - 1]
        .split(" ")
        .filter(|e| e.len() != 0)
        .collect();
    let new_matrix: Vec<Vec<String>> = lines[..lines.len() - 1]
        .iter()
        .map(|l| l.chars().map(|c| c.to_string()).collect())
        .collect();
    let rows = new_matrix.len();
    let cols = new_matrix.iter().map(|e| e.len()).max().unwrap();
    let transpose: Vec<Vec<String>> = (0..cols)
        .map(|col| {
            (0..rows)
                .map(|row| {
                    if col < new_matrix[row].len() {
                        new_matrix[row][col].clone()
                    } else {
                        " ".to_string()
                    }
                })
                .collect()
        })
        .collect();
    let num_strs: Vec<String> = transpose
        .iter()
        .map(|e| e.join("").trim().to_string())
        .collect();
    let mut nums: Vec<Vec<u64>> = vec![];
    let mut numrow: Vec<u64> = vec![];
    for numstr in num_strs.iter() {
        if numstr == "" {
            nums.push(numrow.clone());
            numrow = vec![];
            continue;
        }
        numrow.push(numstr.parse::<u64>().unwrap());
    }
    nums.push(numrow.clone());
    let mut sum: u64 = 0;
    for (idx, oper) in opers.iter().enumerate() {
        let op = if *oper == "+" {
            move |a: u64, b: u64| a + b
        } else {
            move |a: u64, b: u64| a * b
        };
        sum += nums[idx]
            .clone()
            .into_iter()
            .reduce(|acc, s| op(acc, s))
            .unwrap() as u64;
    }
    sum
}
pub fn solve_a(matrix: &Vec<Vec<String>>) -> u64 {
    let nums: Vec<Vec<u64>> = matrix[..matrix.len() - 1]
        .iter()
        .map(|e| e.iter().map(|s| s.parse::<u64>().unwrap()).collect())
        .collect();
    let rows = nums.len();
    let cols = nums[0].len();
    // dbg!(rows, cols);
    let transpose: Vec<Vec<u64>> = (0..cols)
        .map(|col| (0..rows).map(|row| nums[row][col]).collect())
        .collect();
    let opers = &matrix[matrix.len() - 1];
    let mut sum: u64 = 0;
    for (idx, oper) in opers.iter().enumerate() {
        let op = if oper == "+" {
            move |a: u64, b: u64| a + b
        } else {
            move |a: u64, b: u64| a * b
        };
        sum += transpose[idx]
            .clone()
            .into_iter()
            .reduce(|acc, s| op(acc, s))
            .unwrap() as u64;
    }
    sum
}
