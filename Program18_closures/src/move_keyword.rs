pub fn main() {
    let x = vec![1, 2, 3];
    let check = move |num| num == x;
    //below will throw error
    // println!("example of move : {:?}", x);
    let y = vec![1, 2, 3];
    assert!(check(y));
}
