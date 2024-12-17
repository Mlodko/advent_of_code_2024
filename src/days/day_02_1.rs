fn main() {
    let input : String = std::fs::read_to_string("../input").expect("Couldn't read file");
    let count = input.lines()
        .map(parse_line)
        .filter(|nums| check_if_safe(nums))
        .count();
    println!("{}", count);
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|word| word.parse().expect("Couldn't parse input string"))
        .collect()
}

fn check_if_safe(nums : &[u32]) -> bool {
    // 1 if ascending, -1 if descending
    let ascending_descending : i64 = (i64::from(nums[1]) - i64::from(nums[0])).signum();

    nums.windows(2)
        .map(|window| i64::from(window[1]) - i64::from(window[0]))
        .all(|diff| diff.abs() > 0 && diff.abs() < 4 && diff.signum() == ascending_descending)
}
