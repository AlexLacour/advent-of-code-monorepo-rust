pub fn display<T: std::fmt::Debug>(result: T, title: &str) {
    println!("=== {title} ===");
    println!("-> {:?}", result);
    println!("===")
}
