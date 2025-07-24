use unicode_segmentation::UnicodeSegmentation;

pub fn sample_strings() {
    let a1: String = String::new();
    let a2: &'static str = "string";
    let a3: String = String::from("string");
    let a4: String = a3.to_string();
}

pub fn append_to_string() {
    let mut a1 = String::from("hello");
    a1.push_str("World"); //string
    a1.push('.'); //character
    println!("{}", a1)
}
pub fn append_with_without_ownership() {
    let s1: String = String::from("hello ");
    let s2: &'static str = "world";
    let s3: String = s1 + &s2;
    println!("{}", s3);
    println!("{}", s2);
    // println!("{}", s1);//get an error coz transferred owership and thrn trying to print

    let s4: String = format!("{} {}", s2, s3);
    println!("{}", s4);
}
pub fn indexing_in_string() {
    let s1 = String::from("नमस्ते");
    for i in s1.bytes() {
        //print bytes
        println!("{}", i)
    }
    for i in s1.chars() {
        //print chars
        println!("{}", i)
    }
    for i in s1.graphemes(true) {
        //print chars as it is
        println!("{}", i)
    }
}
