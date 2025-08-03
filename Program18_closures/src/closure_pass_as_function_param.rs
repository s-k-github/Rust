fn apply<F>(f: F)
where
    F: Fn(i32) -> i32,
{
    println!("function as closure : {}", f(10)) //20
}
pub fn main() {
    let double = |x| x * 2;
    apply(double);
}

pub fn fn_as_param() {
    fn f_fn(f: impl Fn() -> ()) {
        f();
        f();
    }
    //another way of writing
    fn f_fn1<F: Fn() -> ()>(f: F) {
        f();
        f();
    }
    //using generic same thing
    fn f_fn2<T, F: Fn() -> T>(f: F) {
        f();
        f();
    }
    let s = String::from("hello");
    let f = || println!("fn : {:?}", s);
    f_fn(f);
    f_fn1(f);
    f_fn2(f);
    println!(" s : {:?}", s)
}
pub fn fnmut_as_param() {
    fn f_fn_mut<F: FnMut()>(mut f: F) {
        f();
        f();
    }
    let mut vec = vec![0];
    let mut f = || vec.push(1);
    f_fn_mut(f);
    println!("FnMut : {:?}", vec);
}
pub fn fnonce_as_param() {
    fn f_fn_once<F: FnOnce()>(f: F) {
        f();
        //f(); //will throw error:use of moved value: `f`
    }
    let vec = vec![0];
    let f = move || println!("FnOnce : {:?}", vec);
    f_fn_once(f);
    // f_fn_once(f);//error due to move. if remove error will gp away. use of moved value: `f`
}
pub fn closure_as_fun() {
    fn f_fn_string() -> impl Fn() -> String {
        let s = "World".to_string();
        move || {
            println!("s : {}", s);
            //s//will throw error:cannot move out of `s`, a captured variable in an `Fn` closure
            // move occurs because `s` has type `String`, which does not implement the `Copy`
            s.clone()
        }
    }
    let closure = f_fn_string();
    println!("{:?}", closure());
}
pub fn closure_as_fun_mut() {
    fn f_fn_mut() -> impl FnMut() -> String {
        let mut s = "Hello".to_string();
        move || {
            s += " World";
            s.clone()
        }
    }
    println!("{:?}", (f_fn_mut())());
}
pub fn closure_as_fun_once() {
    fn f_fn_once() -> impl FnOnce() -> String {
        let s = "Hello from FnOnce".to_string();
        move || {
            println!("s: {}", s);
            s
        }
    }
    let s = f_fn_once();
    let f = s();
    // let f1 = s(); //error:use of moved value: `s`
}
