use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file");
    let (left, right) = read_input(&input);
    let mut occurences : HashMap<u32, u32> = HashMap::new();
    left.iter()
        .for_each(|num| {
            if occurences.contains_key(num) {
                return;
            }
            let count: u32 = right.iter()
                .filter(|x| x == &num)
                .count()
                .try_into().unwrap();
            occurences.insert(*num, count);
        });
    let score : u32 = left.into_iter()
        .map(|num| {
            num * occurences.get(&num).unwrap()
        })
        .sum();

    println!("{}", score);
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