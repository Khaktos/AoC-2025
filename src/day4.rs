fn convert(matrix: &Vec<Vec<String>>) -> Vec<Vec<i32>> {
    let size = matrix.len();
    let neighbors: Vec<(i32, i32)> = vec![
        (1, 1),
        (0, 1),
        (-1, 1),
        (1, 0),
        (-1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];
    let mut nums = vec![vec![0; size]; size];
    for x in 0..size {
        for y in 0..size {
            if matrix[x][y] != "@" {
                nums[x][y] = 99;
            }
            for n in neighbors.iter() {
                let (dx, dy) = n;
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx < 0 || nx >= size as i32 {
                    continue;
                }
                if ny < 0 || ny >= size as i32 {
                    continue;
                }
                if matrix[nx as usize][ny as usize] == "@" {
                    nums[x][y] += 1;
                }
            }
        }
    }
    nums
}

pub fn solve_b(matrix: &Vec<Vec<String>>) -> i32 {
    let size = matrix.len();
    let mut mut_matrix = matrix.clone();
    let mut sum = 0;
    loop {
        let prevsum = sum;
        let num_matrix = convert(&mut_matrix);
        for x in 0..size {
            for y in 0..size {
                if num_matrix[x][y] < 4 {
                    sum += 1;
                    mut_matrix[x][y] = ".".to_string();
                }
            }
        }
        if prevsum == sum {
            break;
        }
    }
    sum
}
pub fn solve_a(matrix: &Vec<Vec<String>>) -> i32 {
    convert(matrix)
        .into_iter()
        .map(|row| row.iter().map(|e| if *e < 4 { 1 } else { 0 }).sum::<i32>())
        .sum::<i32>()
}
