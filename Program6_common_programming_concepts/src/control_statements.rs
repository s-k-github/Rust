pub fn main() {
    println!("if else --------------------------------------------------------------------");
    let role: String = "VP".to_lowercase();
    if role == "admin" {
        println!("Admin");
    } else if role == "vp" {
        println!("VP");
    } else {
        println!("No access to this chapter");
    }
    println!("loop --------------------------------------------------------------------");
    loop {
        println!("loop printed");
        break;
    }
    println!(
        "return from loop --------------------------------------------------------------------"
    );
    let mut counter = 0;
    let count = loop {
        counter += 1;
        if counter == 5 {
            break counter;
        }
    };
    println!("Loop ran till count's value is : {}", count);
    println!("while loop --------------------------------------------------------------------");
    while counter <= 10 {
        counter += 1;
        println!("Counter : {}", counter);
    }
    println!("for loop --------------------------------------------------------------------");
    let a = [10, 20, 30, 40, 50];
    for n in a.iter() {
        println!("First for loop: {}", n);
    }
    for n in 2..7 {
        println!("Second for loop: {}", n);
    }
}
