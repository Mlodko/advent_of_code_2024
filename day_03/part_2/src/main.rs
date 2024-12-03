use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("../input").expect("Couldn't read file").replace("\n", "");

    let result : i32 = find_matches(&input).iter()
        .map(|mat| parse_multiply(mat))
        .sum();

    println!("{}", result);
}

fn find_matches(input : &str) -> Vec<&str> {
    let re_mul = Regex::new(r"mul\(\d+,\d+\)").expect("Couldn't create regex");
    let re_do = Regex::new(r"(do\(\)|^)(.*?)(?:don't\(\)|$)").expect("Couldn't create regex");
    re_do.find_iter(input)
        .flat_map(|do_fragment| re_mul.find_iter(do_fragment.as_str()))
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