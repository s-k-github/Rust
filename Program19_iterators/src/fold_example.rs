pub fn main() {
    //fold similar to reduce
    let vector: Vec<i32> = vec![1, 2, 3, 4];
    let total: i32 = vector.iter().fold(0, |acc, &x| acc + x);
    println!("Total of  {:?} is {:?}", vector, total)
}
