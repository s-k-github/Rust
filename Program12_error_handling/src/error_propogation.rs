use std::{
    fs::{self, File},
    io::{self, Read},
};

pub fn error_propogation_read_file_data() -> Result<String, std::io::Error> {
    // let mut s: String = String::new();
    // File::open("Hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    //short code for above
    Ok(fs::read_to_string("Hello.txt")?)
}

fn main() -> Result<String, std::io::Error> {
    Ok(fs::read_to_string("Hello.txt")?)
}
