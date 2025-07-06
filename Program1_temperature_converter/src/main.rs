use std::io;
fn main(){
    println!("Temperature Converter");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    println!("Please select an option 1 or 2 : ");

    // take input from user
    let mut choice=String::new();
    io::stdin().read_line(&mut choice).expect("Please enter valid input");

    //convert to int and check if number
    let choice:u32=match choice.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("Invalid choice. Please enter 1 or 2");
            return;
        }
    };

    if choice == 1{
        celsius_to_fahrenheit();
    }else if choice==2{
        fahrenheit_to_celsius();
    }else{
        println!("Invalid choice. Please enter 1 or 2");
    }
}//close fn main
fn celsius_to_fahrenheit(){
    println!("Please enter temperature in Celsius");
    let temp=take_input();
    let result=(temp*9.0/5.0)+32.0;
    println!("{:.2}C is {:.2}F",temp,result);
}

fn fahrenheit_to_celsius(){
    println!("Please enter temperature in Fahrenheit");
    let temp=take_input();
    let result=(temp-32.0)*5.0/9.0;
    println!("{:.2}F is {:.2}C",temp,result);
    
}
fn take_input()->f64{
    let mut temp=String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");
    match temp.trim().parse::<f64>(){
        Ok(num)=>num,
        Err(_)=>{println!("Invalid input.");
            0.0} 
    }
}