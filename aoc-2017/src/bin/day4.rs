use std::{collections::HashSet, path::Path};
use itertools::Itertools;
use aoc_utils::{console, file_io};

fn is_passphrase_valid(passphrase: &Vec<String>, anagram_condition: bool) -> bool {
    let mut local_passphrase: Vec<String> = passphrase.clone();
    if anagram_condition {
        local_passphrase = local_passphrase.iter().map(
            |s| s.chars().sorted().collect::<String>()
        ).collect();
    }

    let passphrase_set: HashSet<&String> = HashSet::from_iter(local_passphrase.iter());
    //
    passphrase_set.len() == local_passphrase.len()
}

fn main() {
    let input_passphrases_raw = file_io::read_input_file(Path::new(file!()));
    let input_passphrases = file_io::parse_input_string::<String>(input_passphrases_raw, " ");

    let mut n_valid = 0;
    let mut n_valid_with_anagram = 0;
    for passphrase in input_passphrases {
        if is_passphrase_valid(&passphrase, false) {
            n_valid += 1;
        }
        if is_passphrase_valid(&passphrase, true) {
            n_valid_with_anagram += 1;
        }
    }

    console::display(n_valid, "Part 1");
    console::display(n_valid_with_anagram, "Part 2");
}
