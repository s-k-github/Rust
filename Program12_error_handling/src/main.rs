use std::{error::Error, fs};

mod error_handling;
mod error_propogation;
fn main() -> Result<(), Box<dyn Error>> {
    error_handling::non_ideal_way_of_handling_err(23, 2);
    println!("{:?}", error_handling::ideal_way1(23, 2));
    error_handling::open_file();
    error_handling::create_file_if_not_present_instead_of_crash();
    error_handling::expect_function();

    println!(
        "{:?}",
        error_propogation::error_propogation_read_file_data()
    );
    fs::read_to_string("Hello.txt")?;
    Ok(())
}
