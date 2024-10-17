use std::fs;
use std::path::Path;


pub fn read_input_file(script_path: &Path) -> String {
    let mut day_stem = String::from(script_path.file_stem().expect("Error getting the file stem").to_str().unwrap());
    let data_dir = script_path.ancestors().nth(3).unwrap_or(Path::new("")).join("data/inputs");

    day_stem.push_str(".txt");
    let input_file_path = data_dir.join(day_stem);

    let input_content = fs::read_to_string(input_file_path).expect("The input file was not openable");
    let input_content = input_content.trim().to_string();

    // return
    input_content
}
