mod ownership_in_functions;
mod reference_ownership;
mod slice_reference;
fn main() {
    let a = 2;
    let b = a; // ownership of the string moves to b

    println!("{}", a);

    let string1: &str = "hello";
    let string1_1 = string1;
    println!("{}", string1); //no error

    let string2: String = String::from("hello");
    let string2_1 = string2;
    println!("{}", string2); //error here because moving string;default behaviour

    let string3: String = String::from("hello");
    let string3_1 = string3.clone();
    println!("{}", string3); //no error here because copying string

    println!(
        "Ownership in functions------------------------------------------------------------------------"
    );
    let string4: String = String::from("hello");
    ownership_in_functions::taking_ownership(string4);
    let string4_1 = string4; //throw error

    //solution to above
    let string4: String = String::from("hello");
    ownership_in_functions::taking_ownership_as_reference(&string4); //passing reference; references are immutable
    let string4_1 = string4; //throw error

    let string4: String = String::from("hello");
    let result = ownership_in_functions::taking_ownership_and_returning(string4);
    println!("{}", result);

    println!(
        "reference are immutable but solution------------------------------------------------------------------------"
    );
    let mut string4: String = String::from("hello");
    ownership_in_functions::not_taking_ownership_and_updating(&mut string4);
    println!("{}", string4);

    reference_ownership::multiple_mutable_reference();
    reference_ownership::multiple_mutable_immutable_reference();
    reference_ownership::scope_ends_then_new_mutable_reference_possible();
    let dangling_pointer = reference_ownership::dangling_pointer();
    slice_reference::main();
    slice_reference::string_slice();
}
