pub struct Breakfast {
    toast: String,
    fruit: String,
}
impl Breakfast {
    pub fn add_item(toast: &str) -> Breakfast {
        println!("Added item : {}", toast);
        Breakfast {
            toast: String::from(toast),
            fruit: String::from("Peach"),
        }
    }
}
pub fn add_to_wishlist() {
    println!("Added to waitlist");
}
fn seat_at_table() {
    println!("seated at the table")
}
