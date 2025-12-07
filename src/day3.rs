// 3121910778619
// 2912021925339
pub fn solve_b(lines: &Vec<String>) -> i64 {
    let mut sum = 0;
    for line in lines.iter() {
        let digits: Vec<u32> = line.chars().map(|d| d.to_digit(10).unwrap()).collect();
        //find number
    }
    sum
}
// fn find_number(digits: &Vec<u32>, target_len: u32) -> i32 {
//     let mut right_mask = target_len as usize;
//     digits[0..right_mask];
//     0
// }
pub fn solve_a(lines: &Vec<String>) -> u32 {
    let mut sum = 0;
    for line in lines.iter() {
        let digits = line.chars().map(|d| d.to_digit(10).unwrap());
        let first_big = digits.clone().take(line.len() - 1).max().unwrap();
        let pos = digits.clone().position(|e| e == first_big).unwrap();
        let second_big = digits.skip(pos + 1).max().unwrap();
        sum += first_big * 10 + second_big;
    }
    sum
}
