use std::{fs::File, io::ErrorKind};

pub fn non_ideal_way_of_handling_err(a: i32, b: i32) {
    if b == 0 {
        panic!("Can't divide by 0");
    } else {
        println!("{}/{}={}", a, b, a / b)
    }
}

pub fn ideal_way1(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Can't divide by 0".to_string())
    } else {
        Ok(a / b)
    }
}
pub fn open_file() {
    let file_data: Result<File, std::io::Error> = File::open("open.txt");

    match file_data {
        Ok(file) => println!("{:?} File opened", file),
        Err(err) => println!("Can't spot the file: {}", err),
    }
}

pub fn create_file_if_not_present_instead_of_crash() {
    let file_data: Result<File, std::io::Error> = File::open("open.txt");

    match file_data {
        Ok(file) => println!("{:?} File opened", file),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("open.txt") {
                Ok(file) => println!("{:?} File created", file),
                Err(err) => panic!("Can't create the file: {}", err),
            },
            other_error => panic!("Problem found : {:?}", other_error),
        },
    }
}

pub fn expect_function() {
    let file_data: File = File::open("open.txt").expect("Failed to open file open.txt");
    println!("{:?}", file_data)
}
