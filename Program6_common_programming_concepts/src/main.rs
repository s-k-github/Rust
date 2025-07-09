mod compound_types;
mod control_statements;
mod data_types;
fn main() {
    //concept 1 : immutability
    let x = 1;
    println!("Value of x : {}", x);
    //x = 2; // will throw error
    println!("Value of x after changing: {}", x);
    println!("--------------------------------------------------------");

    //concept 2 : make variable mutable using mut keyword
    let mut y = 1;
    println!("Value of x : {}", y);
    y = 2; // will throw error
    println!("Value of x after changing: {}", y);
    println!("--------------------------------------------------------");

    //concept 3 : Shadowing ; creating new variable (could be of different type) using existing name;reference from concept 1
    let x = 2;
    println!("Value of x : {}", x);
    let x: &'static str = "supriya"; // : &'static str is optional to write; write it for type understanding
    println!("Value of x after changing: {}", x);
    println!("--------------------------------------------------------");

    //concept 4 : constant; giving type is mandatory; has to be in capital letters; cant make it mutable
    const MY_VARIABLE: u32 = 100_00;
    println!("Constant : {}", MY_VARIABLE);
    println!("--------------------------------------------------------");

    data_types::main();
    compound_types::main();
    control_statements::main();
}
