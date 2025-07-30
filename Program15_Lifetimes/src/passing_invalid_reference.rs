pub fn passing_invalid_reference_from_function() {
    let string3 = String::from("supriya");
    let longest;
    let longest_str;
    {
        let string2 = String::from("Jadhav");
        longest = longest_string(&string3.as_str(), &string2.as_str());
        longest_str = longest_string_fix(&string3.as_str(), &string2.as_str());
    }
    println!("Longest string : {}", longest);
    println!("Longest string : {}", longest_str);
}
fn longest_string<'a>(s1: &'a str, s2: &str) -> &'a str {
    return String::from("Hello").as_str();
    //line throws error as
    //cannot return value referencing temporary value
    // returns a value referencing data owned by the current function
}
//solution
fn longest_string_fix<'a>(s1: &'a str, s2: &str) -> String {
    return String::from("Hello");
}
//above works coz we are transfering ownership
