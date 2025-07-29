pub fn largest_of_vector() {
    let vec1: Vec<i32> = vec![10, 30, 20, 25];
    println!("Largest number is {:?}", largest_of_num_vector(&vec1));
    let vec2: Vec<char> = vec!['a', 'b', 'z'];
    println!("Largest number is {:?}", largest_of_generic(vec1));
    println!("Largest char is {:?}", largest_of_generic(vec2));
}

//below function will only find largest number
fn largest_of_num_vector(vec1: &Vec<i32>) -> i32 {
    let mut largest = &vec1[0];
    for number in vec1 {
        if number > largest {
            largest = number;
        }
    }
    *largest
}

//traits is used which will be covered in next chapter. T is generic type
fn largest_of_generic<T: PartialOrd + Copy>(vec1: Vec<T>) -> T {
    let mut largest = vec1[0];
    for number in vec1 {
        if number > largest {
            largest = number;
        }
    }
    largest
}
