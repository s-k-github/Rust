run cargo new projectname to create projectname
use std:io; will import input output module from standard rust standard library 
execution starts from main function
mut created mutable empty variable since by default everything is immutable
io::stdin() gives access to input string
read_line() reads line and stores in the variable
.expect will run when read_line encounters and error. then program will panic and run this code.
match temp.trim().parse::<f64>(){
        Ok(num)=>num,
        Err(_)=>{println!("Invalid input.");
            0.0} 
    }
above will parse input in float 64bit. if valid return it else enter error