Dangling reference:
    its a reference that points to invalid data
borrow checker:
    The Rust borrow checker is the system that ensures memory safety without a garbage collector. It enforces ownership, borrowing, and lifetimes at compile time to prevent common bugs like dangling pointers, data races, or double frees.
1. Ownership
    Each value in Rust has a single owner. When the owner goes out of scope, the value is dropped.
    let s1 = String::from("hello");
    let s2 = s1; // ownership moves
    // println!("{}", s1); //  error: s1 has been moved


2. Borrowing
    You can borrow a value instead of transferring ownership:
    fn print_str(s: &String) {
        println!("{}", s);
    }

    let s = String::from("hello");
    print_str(&s); //  borrowed, not moved
    println!("{}", s); //  still valid

    &T = immutable borrow (read-only)
    &mut T = mutable borrow (read/write, exclusive)

3. Borrowing Rules
    You can have one mutable reference OR any number of immutable references, but not both.
    Mutable and immutable references can’t coexist.
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; //  error: cannot borrow `s` as mutable because it’s already borrowed as immutable

4. Dangling References (Prevented by borrow checker)
    fn dangle() -> &String {
        let s = String::from("hello");
        &s //  error: `s` will be dropped, leaving a dangling reference
    }


//////////////////////////////////////////////////////////////////////////////////////////
the lifetime of our return value always has to be tied to the lifetime of one of our parameters
i.e. when we pass a reference from a function it has to be reference that is passed in and thats because we can't return reference that is created inside the function.

//////////////////////////////////////////////////////////////////////////////////////////
Lifetime Elision Rules:
1. Each input reference gets its own lifetime parameter
    fn foo(x: &str, y: &str); 
    // becomes: fn foo<'a, 'b>(x: &'a str, y: &'b str)
2. If there is exactly one input lifetime, it is assigned to all output lifetimes
    fn bar(x: &str) -> &str; 
    // becomes: fn bar<'a>(x: &'a str) -> &'a str
3. If there are multiple input lifetimes, and one of them is &self or &mut self, then that lifetime is assigned to all output lifetimes
    impl MyType {
        fn get_ref(&self, s: &str) -> &str; 
        // becomes: fn get_ref<'a>(&'a self, s: &str) -> &'a str
    }

When You Do Need Lifetimes?
In structs or when returning references from multiple inputs:
    // Compiler needs help choosing which lifetime to return
    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() { x } else { y }
    }
    // Error! Ambiguous output lifetime.

Fix:
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
//////////////////////////////////////////////////////////////////////////////////////////
Static Lifetime:
    reference could live in the duration of the program.
