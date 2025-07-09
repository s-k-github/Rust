pub fn main() {
    println!("typles--------------------------------------------------------------------");
    //type1: tuples
    let person: (&str, i32, f64) = ("Supriya Kolhe", 28, 5.2);
    println!("Name : {}", person.0);
    println!("Age : {}", person.1);
    println!("Height : {}", person.2);

    //deconstruct tuple
    let (name, age, height) = person;
    println!("array--------------------------------------------------------------------");
    //type2: array
    let scores: [i32; 4] = [12, 23, 45, 56];
    println!("Score in math is {}", scores[0]);
    println!("Total subjects under this student are {}", scores.len());
    for n in scores.iter() {
        println!("Score : {}", n)
    }
    // Initialize all elements to the same value
    let repeated = [0; 5]; // [0, 0, 0, 0, 0]
    println!("Repeated array: {:?}", repeated);
    println!("slice--------------------------------------------------------------------");
    //type2: Slices (not distinct)
    let scores = [12, 432, 45, 56, 23];
    let first_3_subjects = &scores[1..4]; //432,45,56
    println!("Scores : {:?}", first_3_subjects);
}
