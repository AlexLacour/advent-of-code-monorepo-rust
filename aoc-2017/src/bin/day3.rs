use std::collections::HashMap;

use aoc_utils::console::display;
use aoc_utils::spatial::{Distance, Point2d};

fn get_manhattan_distance_to(target: i32) -> i32 {
    let mut lateral_distance: i32 = 0;
    let mut vertical_distance: i32 = 0;
    let mut reached_number: i32 = 1;

    while reached_number < target {
        reached_number += 8 * lateral_distance;
        lateral_distance += 1;
        vertical_distance += 1;
    }

    let mut reached_point: Point2d<i32> = Point2d {
        x: (lateral_distance - 1),
        y: (vertical_distance - 1),
    };
    let side_len = 2 * reached_point.x;

    let modes = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut mode_index: usize = 0;
    for i in 0..reached_number - target {
        if i > 0 && i % side_len == 0 {
            mode_index = (mode_index + 1) % modes.len();
        }

        reached_point.x += modes[mode_index].0;
        reached_point.y += modes[mode_index].1;
    }

    reached_point.manhattan_distance(&Point2d::new())
}

fn spiral_sum_sequence(stop: i32) -> Vec<i32> {
    let mut spiral_space: HashMap<(i32, i32), i32> = HashMap::new();

    let mut coords_point: Point2d<i32> = Point2d::new();
    spiral_space.insert(coords_point.to_tuple(), 1);

    let mut i = 0;
    'main_loop: loop {
        let s = 2 * i + 1;
        i += 1;

        for (ds, di, dj) in [(0, 1, 0), (0, 0, -1), (1, -1, 0), (1, 0, 1)] {
            for _ in 0..s + ds {
                coords_point.x += di;
                coords_point.y += dj;

                let mut values: Vec<i32> = Vec::new();

                for k in coords_point.x - 1..coords_point.x + 2 {
                    for l in coords_point.y - 1..coords_point.y + 2 {
                        let coords_to_get = &(k, l);
                        values.push(*spiral_space.get(coords_to_get).unwrap_or(&0));
                    }
                }

                let new_value = values.iter().sum();

                spiral_space.insert(coords_point.to_tuple(), new_value);

                if new_value > stop {
                    break 'main_loop;
                }
            }
        }
    }

    let mut spiral_sequence: Vec<i32> = spiral_space.values().cloned().collect();
    spiral_sequence.sort();

    //
    spiral_sequence
}

fn main() {
    let target: i32 = 277678;

    let spiral_seq = spiral_sum_sequence(target);

    display(get_manhattan_distance_to(target), "Part 1");

    display(spiral_seq.last().unwrap(), "Part 2");
}
