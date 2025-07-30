//Lifetime Elision Rules:

#[derive(Debug)]
struct Sample<'a> {
    sample_column: &'a str,
}
impl<'a> Sample<'a> {
    fn return_announcement(&self, announcement: &str) -> &str {
        println!("Announcement is : {}", announcement);
        self.sample_column
    }
}
pub fn struct_with_lifetime_annotations() {
    let string1: String = String::from("Hello");
    let first_sentence: &str = string1
        .split('.')
        .next()
        .expect("It is a single sentence without fullstop");
    let i: Sample<'_> = Sample {
        sample_column: first_sentence,
    };
    println!("{:?}", i);
}
