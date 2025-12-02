pub fn solve_b(lines: &Vec<String>) -> i64 {
    let mut sum: i64 = 0;
    for line in lines.iter() {
        let elems: Vec<&str> = line.split('-').collect();
        let start = elems[0].parse::<i64>().unwrap();
        let end = elems[1].parse::<i64>().unwrap();
        for num in start..=end {
            let digits = (num as f32).log10().floor() as u32 + 1;
            'outer: for k in 0..digits / 2 {
                let rep = num % 10_i64.pow(k + 1);
                let repdigit = (rep as f32).log10().floor() as u32 + 1;
                let mut acc = 0;
                for i in 0..(digits / repdigit) {
                    acc += 10_i64.pow(repdigit * i) * rep;
                    if acc == num {
                        sum += num;
                        break 'outer;
                    }
                }
            }
        }
    }
    sum
}
pub fn solve_b_slow(lines: &Vec<String>) -> i64 {
    let mut sum: i64 = 0;
    for line in lines.iter() {
        let elems: Vec<&str> = line.split('-').collect();
        let start = elems[0].parse::<i64>().unwrap();
        let end = elems[1].parse::<i64>().unwrap();
        for num in start..=end {
            let digits = (num as f32).log10().floor() as u32;
            for k in 0..=digits {
                let rep = num % 10_i64.pow(k);
                if rep == num {
                    break;
                }
                let numstr = num.to_string();
                let probe = numstr
                    .split(&rep.to_string())
                    .filter(|e| e.len() > 0)
                    .count();
                if probe == 0 {
                    sum += num;
                    break;
                }
            }
        }
    }
    sum
}
pub fn solve_a(lines: &Vec<String>) -> i64 {
    let mut sum: i64 = 0;
    for line in lines.iter() {
        let elems: Vec<&str> = line.split('-').collect();
        let start = elems[0].parse::<i64>().unwrap();
        let end = elems[1].parse::<i64>().unwrap();
        for num in start..=end {
            let digits = (num as f32).log10().floor() as u32;
            if digits % 2 == 0 {
                continue;
            }
            let exp = digits / 2;
            if num % 10_i64.pow(exp + 1) == num / 10_i64.pow(exp + 1) {
                sum += num
            }
        }
    }
    sum
}
