use std::fs;

fn main() {
    let input : String = fs::read_to_string("input").expect("Couldn't read file");
    let (list_a,list_b) = read_input(&input);
    let result : u32 = get_pairs(list_a, list_b).iter()
        .map(|pair| {
            pair.0.abs_diff(pair.1)
        })
        .sum();
    println!("{}", result);
}

fn get_pairs(mut list_a : Vec<u32>, mut list_b : Vec<u32>) -> Vec<(u32, u32)> {
    list_a.sort();
    list_b.sort();
    list_a.into_iter().zip(list_b).collect()
}

fn read_input(input : &str) -> (Vec<u32>, Vec<u32>) {
    input.lines()
        .map(|line| {
            let mut nums = line.split_whitespace();
            let num_a : u32 = nums.next().expect("Missing number 1 in line").parse().expect("Couldn't parse number 1");
            let num_b : u32 = nums.next().expect("Missing number 2 in line").parse().expect("Couldn't parse number 2");
            (num_a, num_b)
        })
        .unzip()
}