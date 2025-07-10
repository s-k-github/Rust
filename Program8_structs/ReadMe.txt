In Rust, a struct is a user-defined data type that lets you group related data under one name .
Structs are a core part of Rust and are used to create complex data models and support object-like behavior.
It's similar to classes in OOP languages, but without methods or inheritance (Rust uses traits instead).
struct Person {
    name: String,
    age: u32,
    is_employed: bool,
}
Why Use Structs?
    1. Group related data together
    2. Define meaningful and reusable types
    3. Add behavior using impl (methods)
    4. Enable ownership, borrowing, and pattern matching

returning CustomerMaster {
        cust_id,
        cust_name,
        isActive: is_active,
    } is valid here make sure type name and variable name matches though

display trait: means defining how something should be printed
    {:?} and {:#?} could be for object i.e. {:#?} for pritter and other one for one liner
    for custom type only this is not enough
        #[derive(Debug)] //debug is trait and derive allows the complier to give basic implementation to the type
        pub struct CustomerMaster {
            cust_id: i32,
            cust_name: String,
            isActive: bool,
        }
impl(implementation block):
    it is used to define methods (functions associated with a struct, enum, or trait). 
    It brings behavior to your data types.
    It basically binds class with methods
    you can call class method using object i.e. r.rectangleLogic is possible.
When to Use impl
    1. You’re working with custom types (struct, enum)
    2. You want to encapsulate behavior with the data
    3. You want to make methods like person.full_name()
    4. You want to implement traits like Display, Debug, Iterator, etc.
When Free Functions Are Better
    1. The logic is not tied to a type
    2. You’re writing generic utility functions
    3. You want simple top-level tools (e.g. math::pow(x, y))
Final Analogy
    Free function = tool lying around  
    impl method = tool inside a toolbox labeled for a specific object