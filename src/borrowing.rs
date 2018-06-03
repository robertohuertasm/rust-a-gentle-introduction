fn publish(lib: &Vec<String>) {
    println!("I have {} books", lib.len());
}

pub fn test_borrowing() {
    let mut lib = Vec::new();
    lib.push("book 1".to_string());
    lib.push("book 2".to_string());
    publish(&lib);
    lib.push("book 3".to_string());
    {
        let book1 = &lib[0];
        println!("{}", book1);
        // uncomment line below to show mutation locking
        // lib.push("book 4".to_string());
    }
    publish(&lib);
}