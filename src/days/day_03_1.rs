use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("../input").expect("Couldn't read file");
    let result : i32 = find_matches(&input).iter()
        .map(|mat| parse_multiply(mat))
        .sum();

    println!("{}", result);
}

fn find_matches(input : &str) -> Vec<&str> {
    let re = Regex::new(r"mul\(\d+,\d+\)").expect("Couldn't create regex");
    re.find_iter(input)
        .map(|mat| mat.as_str())
        .collect()
}

fn parse_multiply(input: &str) -> i32 {
    // Template = "mul(x,y)"
    let stripped = input.trim_start_matches("mul(").trim_end_matches(")");
    let mut parts = stripped.split(',');
    let x : i32 = parts.next().expect("Missing 1st number").parse().expect("Invalid number");
    let y : i32 = parts.next().expect("Missing 2nd number").parse().expect("Invalid number");

    x * y
}