use std::fs;
use std::path::Path;

pub fn read_input_file(script_path: &Path) -> String {
    let mut day_stem = String::from(
        script_path
            .file_stem()
            .expect("Error getting the file stem")
            .to_str()
            .unwrap(),
    );
    let data_dir = script_path
        .ancestors()
        .nth(3)
        .unwrap_or(Path::new(""))
        .join("data/inputs");

    day_stem.push_str(".txt");
    let input_file_path = data_dir.join(day_stem);

    let input_content =
        fs::read_to_string(input_file_path).expect("The input file was not openable");
    let input_content = input_content.trim().to_string();

    // return
    input_content
}

pub fn parse_input_string<T: std::str::FromStr>(input_content: String, pat: Option<&str>) -> Vec<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let split_by_lines: Vec<String> = input_content
        .split("\n")
        .map(|line| line.to_string())
        .collect();

    let split_by_columns: Vec<Vec<&str>>;
    if pat != None {
        split_by_columns = split_by_lines
        .iter()
        .map(|line| line.split(pat.unwrap()).collect())
        .collect();
    }
    else {
        split_by_columns = split_by_lines
        .iter()
        .map(|line| vec![line.as_str()])
        .collect();
    }

    // Type parsing
    let mut parsed_vector: Vec<Vec<T>> = Vec::new();
    for line_of_char in split_by_columns.iter() {
        let new_line: Vec<T> = line_of_char
            .iter()
            .map(|x| {
                x.to_string()
                    .parse::<T>()
                    .expect("Failed to parse the data")
            })
            .collect();
        parsed_vector.push(new_line);
    }

    // return
    parsed_vector
}
