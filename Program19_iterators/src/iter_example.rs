pub fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    for i in v1.iter() {
        println!("iter: {}", i)
    }

    println!("Got:{:?}", v1)
}
