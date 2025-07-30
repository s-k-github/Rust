pub fn main() {
    let string1 = String::from("supriya");
    {
        let string2 = String::from("Jadhav");
        let longest = longest_string(&string1.as_str(), &string2.as_str());
        println!("Longest string : {}", longest)
    }
}
fn longest_s(s1: &str, s2: &str) -> &str {
    if (s1.len() > s2.len()) { s1 } else { s2 }
}
//now above throws error at &str coz lifetime is not know of s2 as in which one to consider.
//by default generic lifetime i.e. 'a will consider smallest lifetime
fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if (s1.len() > s2.len()) { s1 } else { s2 }
}
pub fn smallest_lifetime_example() {
    let string1 = String::from("supriya");
    let longest;
    {
        let string2 = String::from("Jadhav");
        longest = longest_string(&string1.as_str(), &string2.as_str());
    }
    println!("Longest string : {}", longest);
    //above throws error coz longst's lifetime is of string2's lifetime. hence it can't be accessed outside block coz it only exist in the block
    //to remove above error check below

    let string3 = String::from("supriya");
    let longest;
    {
        let string2 = String::from("Jadhav");
        longest = longest_string_fix(&string3.as_str(), &string2.as_str());
    }
    println!("Longest string : {}", longest)
}
//we remove 'a of which we know return is not gonna be such as string3 has longest lifetime hence we remove 'a from second variable
fn longest_string_fix<'a>(s1: &'a str, s2: &str) -> &'a str {
    if (s1.len() > s2.len()) { s1 } else { s2 }
}
