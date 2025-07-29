#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}
pub fn enum_generic() {
    let integer = Option::Some(30);
    let string = Option::Some("Hello");
    let character = Option::Some('c');
    let floating = Option::Some(30.78);
    println!("{:?}", integer);
    println!("{:?}", string);
    println!("{:?}", character);
    println!("{:?}", floating);
}
