use std::thread;
//example by borrowing
pub fn fnexample() {
    let multiplier = 10;
    let closure = |a: i32| a * multiplier;
    println!("Multiplication is : {}", closure(4));
    println!("Multiplier is : {}", multiplier);
}

//capturing by mutable borrowing
pub fn fnmutexample() {
    let mut multiplier = 10;
    //closure has to be mutable coz mutable variable is used in it
    let mut closure = |a: i32| {
        multiplier += 1;
        a * multiplier
    };
    println!("Multiplier is : {}", multiplier);
}
//capturing by ownership
pub fn fnonceexample() {
    let variable = String::from("Hello");
    let closure = move |a: i32| {
        //to make FnOnce you need to either take ownership or drop value
        let mult = variable; // type1.take ownership inside closure. 
        // drop(variable);//type2.drop variable
        a
    };
    println!("Multiplication is : {}", closure(4));
    // println!("Multiplication is : {}", closure(40));
    //use of moved value: `closure`
    // value used here after move

    // println!("Multiplier :{}", multiplier)
    //abve gived error borrow of moved value: `multiplier`
    // value borrowed here after move
}

pub fn moveexample() {
    let variable = String::from("Hello");
    let closure = thread::spawn(move || println!("{}", variable))
        .join()
        .unwrap();
    println!("{:?}", closure);
    // println!("{}", variable); //throw error borrow of moved value: `variable`
}
