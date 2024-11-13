use aoc_utils::algorithms;
use aoc_utils::{console, file_io};
use std::path::Path;

fn run_cycle(memory: &Vec<i32>) -> Vec<i32> {
    let max_value = memory.iter().max().expect("The memory is empty").to_owned();
    let first_max_id = memory
        .iter()
        .position(|x| *x == max_value)
        .expect("The memory is empty");
    let memory_len = memory.len().to_owned();

    // empty block
    let mut modified_memory = memory.clone();
    modified_memory[first_max_id] = 0;
    for next_id in (first_max_id + 1)..(first_max_id + 1 + max_value as usize) {
        modified_memory[next_id % memory_len] += 1;
    }

    //
    modified_memory
}

fn main() {
    let input_blocks = file_io::read_input_file(Path::new(file!()));
    let input_blocks: Vec<Vec<i32>> = file_io::parse_input_string(input_blocks, Some("\t"));
    let memory: Vec<i32> = input_blocks.clone().get(0).unwrap().clone();

    let (lambda, mu) = algorithms::floyd_cycle_finding(&memory, run_cycle);

    console::display(lambda + mu, "Part 1");
    console::display(lambda, "Part 2");
}
