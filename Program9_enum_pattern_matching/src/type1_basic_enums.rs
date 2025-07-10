enum Color {
    //Start with capital letter
    Red,
    Green,
    Blue,
}
pub fn only_enum() {
    check_color(Color::Red);
}

fn check_color(color: Color) {
    match color {
        Color::Red => println!("Color is red"),
        Color::Green => println!("Color is green"),
        Color::Blue => println!("Color is blue"),
    }
}
//-------------------------------------------------------------------------------------------
enum UmbrellaColor {
    Red(String),
    Green(String),
    Blue(String),
    MonthYear(i32, i32),
}

fn check_color1(color: UmbrellaColor) {
    match color {
        UmbrellaColor::Red(msg) => println!("Color is red"),
        UmbrellaColor::Green(msg) => println!("Color is green"),
        UmbrellaColor::Blue(msg) => println!("Color is blue"),
        UmbrellaColor::MonthYear(month, year) => println!("Month Year is printed"),
    }
}
pub fn enum1() {
    let color = UmbrellaColor::Blue(String::from("Blue"));
    let year = UmbrellaColor::MonthYear(07, 2025);
    println!(
        "-------------------------------------------------------------------------------------"
    );
    check_color1(color);
    check_color1(year);
}
