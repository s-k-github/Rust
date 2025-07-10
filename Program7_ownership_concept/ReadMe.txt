Ownership:
    it is what allows rust to make memory safety guarantees without the use of garbage collector
Most programming languages (like Java, Python, C#) use a garbage collector — a program that runs in the background to clean up unused memory.
Rust does not use a garbage collector.
So how does Rust avoid memory bugs?
It uses a system called Ownership to:
    Know who owns a piece of memory
    Decide when to free it
    Prevent bugs like use-after-free, double free, memory leaks
All done at compile time, before your code even runs!
Example in java:
    String name = "Alice";
    String another = name;
    // Java GC handles memory when variables are no longer used
    //In Java, the garbage collector figures out when to free the "Alice" string.

Example in Rust:
    let name = String::from("Alice");
    let another = name; // ownership moves

    // println!("{}", name); //ERROR: name no longer owns the memory!
    //Rust automatically tracks who owns the memory.
    //When "name" moves to "another", Rust ensures only one owner.
NOTE:INT, float,bool,char,types are copy types. ownership dont apply on them;copy their value when assigned or passed;&str
     Vector,String(String::from("hello)),Box, Customer struct apply ownership
There are two concepts, stack and heap.
stack is faster since it does not have to look for space to store but heap can grow and shrink hence run time storage needs to be searched. 
    hence accessing heap is much slower than stack. copy types are directly stored in stack but others are stored as reference and value stored in heap. hence when tracking program has to see it in stack and check for reference in heap, hence takes longer time

Ownership rules:
    1. each value in rust has a variable thats called its owner.
    2. there can be only one owner at a time.
    3. when the owner goes out of scope the value will be dropped.

Solution to string ownership is return string which moves ownership back to the line i.e. return value
references are immutable
mutable references limitations:
    1. in a perticular scope only one reference is allowed for Example  
        let s=&mut data
        let g=&mut data// will thro error code s is already referencing it
        -this is benefit when data sync comes in scope such as consider 2 pointer to 1 data, 1 of which is to write, there is no way to sync both pointers. hence this is advantage
            for example here one pointer is trying to read the data while another pointer is is in the mid of updating it

    2. you cant have a  mutable reference if immutable reference already exist
Hence Rules of references:
    1. at any given time, one can have either one mutable reference or n number of immutable reference
    2. references must always be valid.
dangling pointer (its a benefit)
        returning reference of such as string. coz scope ends
        A dangling pointer is a pointer that references memory that has been freed or gone out of scope
Slice Reference:
    it lets one reference a contiguous sequence of element within a collection instead of referencing the entire collection.
    slices do not take ownership
    