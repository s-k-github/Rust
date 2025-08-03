What are closures?
    closures are like functions except they are anonymous .
    They could be stored as variables
    They could be used as variables as input params to a functions
    and they capture the variables inside the scope in which they are defined
    no need to menion input parameters. could mention if want to. but it will make code verbose i.e. extra code.
Fn trait?
    the closures that borrows variables from env immutably is Fn trair

FnMut?
    a closure when mutates its environment implements FnMut    

FNOnce?
    this is implemented by closures that takes the ownership of variables
    can be only used once since ownership is passed

| Trait    | Behavior                                                                   | Captures Variables                               | Use Case                                                                                  |
| -------- | -------------------------------------------------------------------------- | ------------------------------------------------ | ----------------------------------------------------------------------------------------- |
| `Fn`     | Immutable closure (does not modify or consume captured variables).         | Captures by immutable reference (`&T`).          | Use when the closure only **reads** from the environment.                                 |
| `FnMut`  | Mutable closure (can modify captured variables).                           | Captures by mutable reference (`&mut T`).        | Use when the closure **modifies** variables in the environment.                           |
| `FnOnce` | Consuming closure (takes ownership of captured variables).                 | Captures by value (**ownership is moved**).      | Use when the closure **takes ownership** and can only be called once.                     |
| `move`   | Forces ownership of all captured variables into the closure’s environment. | Moves all captured variables, regardless of use. | Use when the closure needs to **outlive the current scope** (e.g., threads, async tasks). |

conclusion is function throw error at x coz it can't capture the environment but closure does.
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

key features of closure:
1. anonymous functions
    unnamed function that can be stored in a variable or passed to other function
2. capture environment
    using Fn, FnOnce, FnMut concept
3. type inference
    rust infers the types of parameters and return types in most closures, so explicit type annotations are often unncessary.
4. flexibility
    closures can be stored as function pointers or traits like Fn, FnOnce, FnMut depending on what they do. 

Closure traits:
1. Fn - captures variable by reference (&T)
 Fn (capture by &)
- immutable borrow from environment
- can be called more than once
2. FnMut - captures variables by mutable reference (&mut T)
FnMut (capture by &mut)
- mutable borrow from environment
- can be called more than once
3. FnOnce - captures variables by value(T)
FnOnce (capture by value)
- moves captured values into closure, if needed
- can be called at least once


closure as function parameter: