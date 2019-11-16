// import loops module which is loops.rs
mod loops;
mod functions;

fn hello_world() {
    println!("Hello World!");
}


fn main() {
    hello_world();
    // call public function loops in loops module
    loops::loops();
    functions::functions();

    // both expressions are valid
    // type doesn't have to be mentioned, compiler can decipher
    let _i = 10;
    let _k : i32 = 10;

    // i is immutable
    // i = 11;

    let mut ii = 10;
    println!("Hello, World! {}", ii);
    assert_eq!(ii, 10);
    println!("{} {}", disp(&mut ii), ii);


    let arr = [10,20,30,40];
    for i in 0..4 {
        println!("{}", arr[i]);
    }
    let _slice1 = &arr[1..];
    println!("{:?}, {:?}", arr[3], _slice1.get(5));
}

fn disp(x: &mut i32) -> f64 {
    if *x > 1 {
    *x = 50;
    
    std::f64::consts::PI + 10 as f64
    }
    else {
    *x = 20;
    std::f64::consts::PI
    }

}
