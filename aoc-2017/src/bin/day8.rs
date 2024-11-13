use aoc_utils::{console, file_io};
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input_instructions = file_io::parse_input_string::<String>(
        file_io::read_input_file(std::path::Path::new(file!())),
        Some(" "),
    )
    .to_owned();

    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut max_value_all_time: Option<i32> = None;

    for instruction in input_instructions.iter() {
        // Parsing
        let register = &instruction[0];
        let operation_str = &instruction[1];
        let operation_value = *(&instruction[2].parse::<i32>().expect("nan"));

        let target_register = &instruction[&instruction.len() - 3];
        let condition_op = &instruction[&instruction.len() - 2];
        let condition_value = *(&instruction[&instruction.len() - 1]
            .parse::<i32>()
            .expect("nan"));

        // KeyError Handling
        if !registers.contains_key(register) {
            registers.insert(register.clone(), 0);
        }
        if !registers.contains_key(target_register) {
            registers.insert(target_register.clone(), 0);
        }

        let condition_result: bool = match condition_op.as_str() {
            ">" => registers[target_register] > condition_value,
            "<" => registers[target_register] < condition_value,
            ">=" => registers[target_register] >= condition_value,
            "<=" => registers[target_register] <= condition_value,
            "==" => registers[target_register] == condition_value,
            "!=" => registers[target_register] != condition_value,
            _ => false,
        };

        if condition_result {
            registers.insert(
                register.clone(),
                registers[register]
                    + match operation_str.as_str() {
                        "inc" => operation_value,
                        "dec" => operation_value * -1,
                        _ => 0,
                    },
            );
        }

        let registers_max = registers.values().max().unwrap();
        max_value_all_time = Some(
            match max_value_all_time
                .unwrap_or(registers_max - 1)
                .cmp(registers_max)
            {
                Ordering::Less => *registers_max,
                Ordering::Equal => *registers_max,
                Ordering::Greater => max_value_all_time.unwrap(),
            },
        )
    }

    console::display(registers.values().max().unwrap(), "Part 1");
    console::display(max_value_all_time.unwrap(), "Part 2");
}
