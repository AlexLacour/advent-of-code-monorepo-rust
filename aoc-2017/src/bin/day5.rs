use std::path::Path;

use aoc_utils::{console, file_io};

fn run_maze(jump_maze: &mut Vec<i32>, strange: bool) -> i32 {
    let mut jump_index: i32 = 0;
    let mut n_steps = 0;
    loop {
        let jump_value: Option<i32> = jump_maze.get(jump_index as usize).cloned();

        if jump_value == None {
            break;
        }
        let jump_value = jump_value.unwrap();

        if strange && jump_value >= 3 {
            jump_maze[jump_index as usize] -= 1;
        } else {
            jump_maze[jump_index as usize] += 1;
        }

        jump_index += jump_value;

        n_steps += 1;
    }

    // return
    n_steps
}

fn main() {
    let input_jumps = file_io::read_input_file(Path::new(file!()));
    let input_jumps: Vec<Vec<i32>> = file_io::parse_input_string(input_jumps, " ");

    let jump_maze: Vec<i32> = input_jumps.iter().map(|x| x[0]).collect();

    let n_steps = run_maze(&mut jump_maze.clone(), false);
    let strange_n_steps = run_maze(&mut jump_maze.clone(), true);

    console::display(n_steps, "Part 1");
    console::display(strange_n_steps, "Part 2");
}
