mod advanced_closure;
mod basic_closure;
mod closure_function_behaviour;
mod closure_pass_as_function_param;
mod move_keyword;
fn main() {
    //basic example
    let v: i32 = 1;
    let w: Vec<i32> = vec![1, 2, 3];
    let w2: Vec<i32> = w.iter().map(|x| x + v).collect();
    println!("Basic Example : {:?} ", w2);

    basic_closure::fnexample();
    basic_closure::fnmutexample();
    basic_closure::fnonceexample();
    basic_closure::moveexample();
    advanced_closure::main();
    closure_function_behaviour::main();
    move_keyword::main();
    closure_pass_as_function_param::main();
    closure_pass_as_function_param::fn_as_param();
    closure_pass_as_function_param::fnmut_as_param();
    closure_pass_as_function_param::fnonce_as_param();
    closure_pass_as_function_param::closure_as_fun();
    closure_pass_as_function_param::closure_as_fun_mut();
    closure_pass_as_function_param::closure_as_fun_once();
}
