#![allow(dead_code)]

use days::day_05;

mod days;

fn main() {
    match day_05::solve_part_2("./inputs/day_05_input") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e)
    }
    
}
