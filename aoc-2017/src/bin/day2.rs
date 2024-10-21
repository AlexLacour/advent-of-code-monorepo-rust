use std::cmp::Ordering;
use std::ops::Div;
use std::path::Path;

use aoc_utils::console;
use aoc_utils::file_io;
use aoc_utils::vector_utils;

fn get_minmax_diff_checksum(mat: &Vec<Vec<i32>>) -> i32 {
    let mut checksum: i32 = 0;
    for row in mat.iter() {
        checksum += match row.iter().max() {
            Some(x) => x,
            None => continue,
        } - match row.iter().min() {
            Some(x) => x,
            None => continue,
        };
    }

    checksum
}

fn get_div_checksum(mat: &Vec<Vec<i32>>) -> i32 {
    let mut checksum: i32 = 0;

    for row in mat.iter() {
        let pairs = vector_utils::get_pairs(row);

        let fracts: Vec<f32> = pairs
            .iter()
            .map(|pair| match pair.0.cmp(pair.1) {
                Ordering::Greater => (*pair.0 as f32).div(*pair.1 as f32),
                _ => (*pair.1 as f32).div(*pair.0 as f32),
            })
            .filter(|x| x.fract() == 0.0)
            .collect();

        checksum += fracts[0] as i32;
    }

    checksum
}

fn main() {
    let raw_input_content = file_io::read_input_file(Path::new(file!()));
    let input_mat = file_io::parse_input_string::<i32>(raw_input_content, "\t");

    let checksum = get_minmax_diff_checksum(&input_mat);
    console::display(checksum, "Part 1");

    let checksum = get_div_checksum(&input_mat);
    console::display(checksum, "Part 2");
}
