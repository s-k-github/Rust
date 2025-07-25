panic macro is used.
but it doesnt show where error is present hence we run
$env:RUST_BACKTRACE="1"; cargo run
to clear : Remove-Item Env:RUST_BACKTRACE

Result enum
    The Result enum in Rust is used for error handling in a type-safe way. 
    It’s part of the standard library and represents either success or failure of an operation.
    enum Result<T,E>{
        OK(T),
        ERR(E),
    }

error propogation:
    Error propagation means passing an error up the call stack instead of handling it right away — so that the caller of the function can decide how to handle it.
    this gives more control to the caller to decide what to do with that error
    ? is used for error propogation
    it can only be used in a function and not in main since main does not return anything.
        but we can modify the function to do so
    





