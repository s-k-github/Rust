pub fn multiple_mutable_reference() {
    //restriction 1
    let mut s = String::from("hello");
    let ref1 = &mut s;
    let ref2 = &mut s; //will throw error
    println!("{} {}", ref1, ref2);
    //solution

    let s1 = String::from("hello");
    let ref3 = &s1;
    let ref4 = &s1;
    println!("{} {}", ref3, ref4);
}

pub fn multiple_mutable_immutable_reference() {
    //restriction 2
    let mut s1 = String::from("hello");
    let ref3 = &s1;
    let ref4 = &s1;
    let ref5=&mut s1 //throw error
    println!("{} {} {}", ref3, ref4,ref5);
    
}

pub fn scope_ends_then_new_mutable_reference_possible(){
    let mut s1 = String::from("hello");
    let ref3 = &s1;
    let ref4 = &s1;
    println!("{} {}", ref3, ref4);
    
    let ref5=&mut s1 ;//throw error
    println!("{}", ref5);
}

pub fn dangling_pointer()->&String{
    let s=String::from("hello");
    &s;
}