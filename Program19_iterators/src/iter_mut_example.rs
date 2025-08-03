pub fn main() {
    let mut vector = vec![1, 2, 3, 4];
    for i in vector.iter_mut() {
        *i += 2;
        println!("iter_mut : {}", i);
    }
    println!("Updated vector iter_mut:{:?}", vector)
}
