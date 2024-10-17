pub fn get_pairs<T>(data: &Vec<T>) -> Vec<(&T, &T)> {
    let mut combinations: Vec<(&T, &T)> = Vec::new();
    for (i, x) in data.iter().enumerate() {
        for y in data[i+1..].iter() {
            combinations.push((x, y));
        }
    }

    // return
    combinations
}
