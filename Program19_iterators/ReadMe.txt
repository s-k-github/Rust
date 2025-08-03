allows you to iterate over sequence of elements regardless of how elements are stores
for example array, hadmap, graph, custom data structure
they encapsulate the logic for iterating over these different dependencies

Types of iterators:
1> .iter()
    -purpose: create an iterators that borrows each element in the collection imutably.
    -ownership: yields references (&T), not owned values:Yhe original collection is unchanged.
    -use case: use when you want to read or inspect elements without taking ownership or modifying collection
2> .iter_mut()
    -purpose: borrow each element in a collection mutably.
    -ownership: yields mutable references (&mut T), allowing you to modify the elements in place.
    -use case: Use when you need to change the elements of a collection.
3> .into_iter()
    -purpose: consume the collection and created an iterator that takes ownership of each element.
    -ownership: yields owned values(T). After iteration, the original collection is no longer accessible.
    -use case: use when you want to transfer ownership of the elements.

| Iterator        | Ownership Taken?      | Item Type Yielded  | Can Modify Items? | Common Use Case                                                                |
| ------------- | ----------------------| ------------------ | ----------------- | ------------------------------------------------------------------------------ |
| `iter()`      | No (borrows)          | `&T` (reference)   | No              | When you want to **read items** without modifying or consuming the collection. |
| `iter_mut()`  | No (mutable borrow)   | `&mut T` (mut ref) | Yes             | When you want to **modify items** in-place.                                    |
| `into_iter()` | Yes (takes ownership) | `T` (by value)     | Yes             | When you want to **consume** the collection (e.g., move items into new owner). |

| Iterator      | Yields   | Ownership | Purpose                                  |
| ------------- | -------- | --------- | ---------------------------------------- |
| `iter()`      | `&T`     | Borrow    | Read-only iteration                      |
| `iter_mut()`  | `&mut T` | Mutable   | Allows in-place modification of elements |
| `into_iter()` | `T`      | Ownership | Consumes collection for ownership        |

They give fine-grained control over data ownership and mutability during iteration.

Methods to modify or consume iterators: map(), filter(), fold ()
| Method   | Purpose                                      | Input                        | Output                            | Ownership Impact      |
| -------- | -------------------------------------------- | ---------------------------- | --------------------------------- | --------------------- |
| `map`    | Transforms each item                         | Closure: `Fn(T) -> U`        | New iterator of transformed items | Doesn't consume input |
| `filter` | Keeps items where closure returns `true`     | Closure: `Fn(&T) -> bool`    | New iterator with fewer items     | Doesn't consume input |
| `fold`   | Reduces all items to one value (accumulator) | Closure: `Fn(Acc, T) -> Acc` | Single value (e.g., sum, product) | Consumes the iterator |
| `zip`    | Combines two iterators into pairs (tuples) | Another iterator of same length | Iterator of `(T, U)` tuples      | Borrows or moves based on input |
| `collect`| Converts iterator into a collection        | Iterator                        | `Vec`, `HashMap`, `String`, etc. | Consumes the iterator           |
