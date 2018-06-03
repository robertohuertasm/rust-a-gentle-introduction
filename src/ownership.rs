fn publish(lib: Vec<String>) {
    println!("I have {} books", lib.len());
}

pub fn test_ownership() {
    let mut lib = Vec::new();
    lib.push("book 1".to_string());
    lib.push("book 2".to_string());
    publish(lib);
    // remove comment below to see what happens...
    // lib.push("book 3".to_string());
}