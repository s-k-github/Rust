pub fn main() {
    let r;
    {
        let y = 5;
        r = &y;
        //above r is dangling reference coz r points to y where y's scope ends before using it in printline indirectly
    }
    println!("{}", r);

    //non dangling reference
    let a;
    let y = 5;
    a = &y;
    println!("{}", a)
}
