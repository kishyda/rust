fn apply<F>(f: F) where
F: FnOnce() {
    f();
}

fn apply_to_3<F>(f: F) -> i32 
where F: Fn(i32) -> i32 {
    f(3)
}

fn thang<F>(f: F, x: i32) -> i32 
where F: Fn(i32) -> i32{
    f(x)
}

fn call_me<F: Fn()>(f: F) {
    f();
}

fn call_me2<F: Fn(x) -> i32>(f: F, x: i32) -> i32{
    f(x)
}

fn func() {
    println!("I'm a function!");
}

pub fn function_s() {
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();
    let diary = || {
        println!("I said {}.", greeting);
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");
        std::mem::drop(farewell);
    };
    apply(diary);
    let double = |x: i32| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
    let mut s_input: String = String::new();
    std::io::stdin().read_line(&mut s_input).expect("error reading string");
    let randfunc = |val| {
        let mut result = val;
        for value in 0..=val {
            result *= val;
        }
        result
    };
    let input: i32 = s_input.as_str().trim().parse().expect("err"); 
    println!("{} passed through randfunc is {}", input, thang(randfunc, input))
}

pub fn function() {
    let closure = || println!("I'm a closure!");
    call_me(closure);
    call_me(func);
}
