#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
#[derive(Debug)]
struct Point_Combined<T, U> {
    x: T,
    y: U,
}
pub fn struct_generic() {
    let int_struct = Point { x: 20, y: 34 };
    let float_struct = Point { x: 20.9, y: 34.8 };
    let combined_struct = Point_Combined { x: 20, y: 34.8 };
    println!("{:?}", int_struct);
    println!("{:?}", float_struct);
    println!("{:?}", combined_struct);
}
