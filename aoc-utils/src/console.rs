pub fn display<T: std::fmt::Debug>(result: T, title: String) {
    println!("=== {title} ===");
    println!("-> {:?}", result);
    println!("===")
}
