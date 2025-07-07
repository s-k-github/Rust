use std::env; //access command line arguments
use std::fs::File; //access to file; opens and reads file
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid input. Usage Cargo run <fileName>");
        return;
    }

    let filepath = &args[1];
    println!("Reading file: {}", filepath);

    //attempt to open file
    let mut file = match File::open(filepath) {
        Ok(file) => file,
        Err(_) => {
            println!("Error opening file");
            return;
        }
    };

    //read file content
    let mut content = String::new();
    if let Err(err) = file.read_to_string(&mut content) {
        println!("Error reading file : {}", err);
        return;
    }

    let word_count = count_words(&content);
    println!("Word Count: {}", word_count);
}
fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}
