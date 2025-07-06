use std::io;
fn main(){
    println!("Simple calculator");
    println!("Operators available : +, -, /, %, *");
    println!("Enter expression such as 8 + 2 with spaces in between ");
    println!("Type 'exit' to quit\n");

    loop {
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Enter valid input");
    let input = input.trim();
    if input.eq_ignore_ascii_case("exit") {
            println!("Exiting calculator. Goodbye!");
            break;
        }
    let tokens:Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len()!=3{
        println!("Invalid input");
        continue;
    }

    let num1:f64=match tokens[0].parse(){
        Ok(num)=>num,
        Err(_)=>{println!("Invalid first number");
        continue;
        }
    };
    let num2:f64=match tokens[2].parse(){
        Ok(num)=>num,
        Err(_)=>{println!("Invalid second number");
            continue;
        }
    };

    let operator =tokens[1];

    let result=match operator{
        "+"=>add(num1,num2),
        "-"=>subtract(num1,num2),
        "/"=>divide(num1,num2),
        "*"=>multiply(num1,num2),
        "%"=>modify(num1,num2),
        _=>{println!("Invalid operator");continue;}
    };
    println!("Result is = {:.2}", result);
}
}
fn add(num1:f64,num2:f64)->f64{
    num1+num2
}
fn subtract(num1:f64,num2:f64)->f64{
    num1-num2
}
fn divide(num1:f64,num2:f64)->f64{
    if num2==0.0{
        println!("Divide by 0 is not allowed");
        std::process::exit(1);
     }
    num1/num2
}
fn multiply(num1:f64,num2:f64)->f64{
    num1*num2
}
fn modify(num1:f64,num2:f64)->f64{
    if num2==0.0{
        println!("Divide by 0 is not allowed");
        std::process::exit(1);
     }
    num1%num2
}