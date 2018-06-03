fn add_book(lib: &mut Vec<String>, book: &str) {
    lib.push(book.to_string());
}

pub fn test_mutable_borrowing() {
    let mut lib = Vec::new();
    lib.push("book 1".to_string());
    lib.push("book 2".to_string());
    add_book(&mut lib, "book 3");
    lib.push("book 4".to_string());
    {
        let book1 = &mut lib[0];
        println!("{}", book1);
        // uncomment any of the lines below to check the behavior
        // add_book(&mut lib, "book 5");
        // println!("{}", &lib[0]);
    }

    {
        // let lib2 = &mut lib;
        // println!("{}", lib2[0]);
        println!("{}", lib[0]);
        // try uncommenting the two previous lines and guess what will happen
    }
}