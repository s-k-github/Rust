Println is macro. 
immutability means variables are by default immutable i.e. their state can't be changed by default
mut keyword is use to make variable mutable
shadowing means creating variable with same name which could be of different type.(Scope)
constant cant be mute

by default evrything is private
pub keyword make anything public
we have int,float,bool,char
by default int is i32 i.e. signed 32 bit int

compound datatypes: they are types that group multiple values into one. Unlike scalar types (i32, char, etc.), compound types can hold multiple values of either the same or different types.
The main compound types in Rust are:
    Tuples — Fixed-size, can contain different types
             Tuples are useful for returning multiple values from a function.
    Arrays — Fixed-size, must contain values of the same type
             Arrays are fast, but size must be known at compile time.
    Slices — Dynamically-sized views into arrays or vectors
             Use slices when working with parts of arrays or vectors efficiently.

functions: follow snake case in names
    fn add(x:i32,y:i32){}
    call->add(12,23)
    2 ways to return values
    1. require return type and return keyword, need semi colon
        fn add_number(x:i32,y:i32)->i32{
            return x+y;
        }
    2. doesnt matter if have return type, last line do not contain semicolon and return keyword
        fn add_number(x:i32,y:i32)->i32{
            x+y
        }

control statement: if else, loop keyword,while loop,for loop