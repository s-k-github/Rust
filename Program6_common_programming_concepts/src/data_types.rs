pub fn main() {
    // Integer literals with different bases
    let a: i32 = 98_222; // Decimal
    let b: i32 = 0xff; // Hexadecimal (255)
    let c: i32 = 0o77; // Octal (63)
    let d: i32 = 0b1111_0000; // Binary (240)
    println!("Types of int : {} {} {} {}", a, b, c, d);

    let f: u8 = 255; //making it 256 will give error due to unsigned 8 bit

    //Float
    let b: f64 = 4.5;

    //bool
    let b: bool = 1 == 1;

    //char
    let b: &'static str = "c";
}
