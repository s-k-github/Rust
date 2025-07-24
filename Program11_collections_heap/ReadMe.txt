collections:
    collections are data structures that group multiple values into a single data type.
    They are part of the std::collections module

Vectors:
    growable, heap-allocated array
    values of the same type
    Vectors are useful when the number of elements is not known at compile time or when the list needs to grow or shrink dynamically.
    Declared using vec![] or Vec::new() 
        let vector:Vec<i32>=Vec::new();
    Type-safe and memory-safe
        The compiler ensures that variables and values are always used according to their correct types. For example, you can't accidentally add a String to an i32 — it will fail at compile time.
        Rust guarantees you won’t access invalid memory — no null pointers, dangling references, or use-after-free bugs — because of its strict ownership and borrowing rules, enforced at compile time without a garbage collector.
        get method is presented to safer way to access elements 
    iterate over vector example
    let mut ownership_issue = vec![23, 34, 56, 78];
    for i in &mut ownership_issue {
        *i += 1;
        println!("{}", i);
    }
    56
    24
    35
    57
    79

    storing enum inside vector

String:
    in rust string are stored as collection of UTF 8 encoded bytes.
    here encoded bytes are basically string stored in 1ns and 0s. 
    computer needs to be able to interpret those bytes into characters.
    here UTF8 comes in picture. for that understand ASCII Americal Standard Code Information Interchange
    In ASCII each character is 1B
    Unicode is wide collection of characters and also emojis.
    ASCII is encorporated into Unicode characters i.e. first 128 characters.
    so we can use Unicode encoding to parse ASCII text.

    UTF8 is a variable-width character encoding. 
    called as variable-width coz each character could be represented as 1B,2B,3B,4B.
    here each character could be different in terms of bytes.
    it is most populer encoding of Unicode. hence used in Rust
    .bytes and .chars are provided to iterate but to get exact words we need to import crate
        unicode-segmentation="1.7.1"

Hashmaps:
    stores key value pair of any type
    uses hashing to determine how to store them memory
    use std::collections::HashMap;
    


