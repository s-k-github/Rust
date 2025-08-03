pub fn main() {
    let vector: Vec<i32> = vec![1, 2, 3, 4];
    let squares: Vec<i32> = vector.iter().map(|x: &i32| x * x).collect();
    println!("Squares of {:?} are {:?}", vector, squares)
}
