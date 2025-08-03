pub fn main() {
    let mut vector = vec![1, 2, 3, 4];
    for mut i in vector {
        println!("iter_into : {}", i);
        i += 2;
        println!("iter_into after adding 2: {}", i);
    }
    // println!("iter into after passing ownership : \n{:?}", vector) //error:borrow of moved value: `vector`
}
