Generic types:
    Generics allow you to abstract over types, writing code that works with many concrete types without duplication.
    Generic types let you write flexible, reusable code that works with different data types, while still being type-safe.

    generics, traits and lifetimes are used to avoid code duplication

    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        OK(T),
        Err(E),
    }


    1.impl<U> Point<U>{
    2.    fn x(&self)->&U{
    3.        &self.x
        }
    } 
    1. means, Implement methods for all Point<U> where U can be any type
        This is an implementation block for Point where the type parameter is now called U.
        It's just a different name than T — Rust allows changing the name of the generic when implementing.
    2. This defines a method named x that:
        Takes an immutable reference to the self (i.e., the Point instance),
        And returns a reference to the x field.
    3. let p = Point { x: 10, y: 20 };
        let val = p.x();  // method call with automatic &p
        Rust desugars p.x() into Point::x(&p) automatically — and that only works because &self is the first parameter.

    Why line 2 is written the way it is:
    1. By writing &self, you're saying this method is part of the type itself (Point<U>). It gets special behavior:
        You can call it using . syntax: instance.method().
        It can access fields directly via self.x.
        It's considered a method, not a plain function.
    2. If you used this instead:
        fn x(point_reference: &Point<U>) -> &U {
            &point_reference.x
        }
        Then it's just a normal function, and Rust has no idea it's related to the Point type.
        You'd have to call it like:
        let p = Point { x: 10, y: 20 };
        let val = x(&p); // No dot syntax


                
///////////////////////////////////////////////////////////////////////////////////////





