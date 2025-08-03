pub fn main() {
    let x = 2;
    let check = |num| num == x;
    /*below throw error as
    can't capture dynamic environment in a fn item
    use the `|| { ... }` closure form instead
        */
    // fn check_function(num: i32) -> bool {
    //     num == x
    // }
    let y = 2;
    assert!(check(y));
}
/*conclusion is function throw error at x coz it can't capture the environment but closure does.
closure needs extra memory for the same reason to store their context
closure capture their env in 2 ways which directly map to the 3 ways a function can take in input param
1. by taking ownership
    FnOnce takes ownership of variables inside the closure env.
    The once part explain the fact that closures can't take ownership of the same variable more than once.
    So these closures can be only called once.
2. by borrowing values mutably
    FnMut
3. by borrowing values immutably
    Fn

When we create a closure rust infers which of these traits to use based on how you use the values inside the closures environment.
We could force clousre to take ownership of the values it uses inside its environment by using the move keyword in front of the closure
that is when you are passing closure from one thread to another thread so we can also pass ownership of the variables from one thread to another thread
*/
