pub fn solve_b(lines: &Vec<String>) -> i32 {
    let mut curr_pos = 50;
    let mut zeros = 0;
    for line in lines.iter() {
        let num = line[1..].parse::<i32>().unwrap();
        let mut spins = 0;
        if num > 100 {
            spins = num / 100;
        }
        let rot = num % 100;
        let start = curr_pos;
        if line.starts_with('R') {
            curr_pos += rot;
        } else {
            curr_pos -= rot;
        }
        zeros += spins;
        if curr_pos > 100 {
            zeros += 1
        }
        curr_pos %= 100;
        if curr_pos < 0 {
            if start != 0 {
                zeros += 1
            }
            curr_pos += 100;
        }
        if curr_pos == 0 {
            zeros += 1;
        }
    }
    zeros
}

pub fn solve_a(lines: &Vec<String>) -> i32 {
    let mut curr_pos = 50;
    let mut zeros = 0;
    for line in lines.iter() {
        let num = line[1..].parse::<i32>().unwrap();
        if line.starts_with('R') {
            curr_pos += num;
        } else {
            curr_pos -= num;
        }
        curr_pos %= 100;
        if curr_pos == 0 {
            zeros += 1;
        }
    }
    zeros
}
