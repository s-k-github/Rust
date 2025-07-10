pub fn main() {
    let mut s = String::from("hello");
    let a = calling_fn(&s);
}
fn calling_fn(some_word: &String) -> usize {
    let bytes = some_word.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    bytes.len()
}
pub fn string_slice() {
    let data = String::from("Hello World");
    let hello = &data[0..5]; //[..5] is also valid
    let world = &data[6..11]; //[6..] is also valid
    println!("{}", return_slice(&data))
}
fn return_slice(some_word: &String) -> &str {
    let bytes = some_word.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &some_word[0..i];
        }
    }
    &some_word[..]
}
