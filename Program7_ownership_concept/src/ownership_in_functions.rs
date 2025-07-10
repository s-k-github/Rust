use std::string;

pub fn taking_ownership(some_string: String) {
    println!("{}", some_string)
}
pub fn taking_ownership_and_returning(some_string: String) -> String {
    some_string
}

pub fn taking_ownership_as_reference(some_string: &String) {
    println!("{}", some_string)
}
pub fn not_taking_ownership_and_updating(some_string: &mut String) {
    some_string.push_str(" World");
}
