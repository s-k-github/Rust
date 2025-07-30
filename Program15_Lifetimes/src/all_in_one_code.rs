use std::fmt::Display;
pub fn all_in_one_code<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    print!("Announcement: {}", ann);
    if (x.len() > y.len()) { x } else { y }
}
