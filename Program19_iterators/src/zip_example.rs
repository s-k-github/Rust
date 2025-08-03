use std::collections::HashMap;

//useful when you want to iterate over 2 iterator and for each iteration you want to pair them together
pub fn main() {
    //d will be omitted since nothing to pair with
    let keys: Vec<String> = vec!["a", "b", "c", "d"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let vals: Vec<u32> = vec![1, 2, 3];
    // let zipped: Vec<(String, u32)> = keys.into_iter().zip(vals.into_iter()).collect();
    let zipped: HashMap<String, u32> = keys.into_iter().zip(vals.into_iter()).collect();
    println!("Zipped : {:?}", zipped)
}
