use std::path::Path;

use aoc_utils;

fn get_sum(integers: &Vec<u32>, look_further: bool) -> u32 {
    let mut sum = 0;
    for (number_index, number) in integers.iter().enumerate() {
        if look_further && *number == integers[(number_index + (integers.len() / 2)) % integers.len()]
        || !look_further && *number == integers[(number_index + 1) % integers.len()] {
            sum += number;
        }
    }
    // return
    sum
}

fn main() {
    let input_file_content = aoc_utils::read_input_file(Path::new(file!()));
    
    let input_integers: Vec<u32> = input_file_content.chars().map(|c| c.to_digit(10).ok_or(0).unwrap()).collect();
    
    // Part 1
    let sum = get_sum(&input_integers, false);
    println!("{}", sum);

    // Part 2
    let sum = get_sum(&input_integers, true);
    println!("{}", sum);
}