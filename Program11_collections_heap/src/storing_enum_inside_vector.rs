pub fn fn_storing_enum_inside_vec() {
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(2.3),
        SpreadSheetCell::Text(String::from("blue")),
    ];
    match row.get(20) {
        Some(SpreadSheetCell::Float(i)) => println!("{} is float", i),
        Some(SpreadSheetCell::Text(i)) => println!("{} is text", i),
        Some(_) => println!("Not a integer!"),
        None => println!("Index out of bound"),
    }
}
