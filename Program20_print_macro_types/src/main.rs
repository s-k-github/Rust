use std::io::Write;

fn main() {
    print!("Hello\n there"); // use buffer to print hence takes time to print and error does not use buffer hence print quickly
    print!("Hello");
    std::io::stdout().flush().unwrap(); //this is used to print error at the moment it whould be printed.
    eprint!("error");
    eprintln!("error in line");
    println!("Hello there");
    let username = "supriya";
    let age = 23;
    let formatexample = format!("Username is  {} with age {}", username, age);
    let formatexample1 = format!(
        "Username is  {user} with age {totalage}",
        user = username,
        totalage = age
    );
    println!("{}", formatexample);
    println!("{}", formatexample1);
}
