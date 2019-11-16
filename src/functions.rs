pub fn functions() {

    let mut x : i32 = 10;
    // &mut has to be used to imply that x can be modified inside the function
    disp(&mut x);

}

// types for arguments have to be mentioned
// & keyword for passing by reference, no new value is created, but can't modify inside the function
// &mut keyword to be able to modify the value inside the function
// -> for return type
// last expression in the function is the returned value
// return should be only used for conditional returns, not as last statement
fn disp(x: &mut i32) -> f64 {
    if *x > 1 {
    *x = 50;
    
    // return value
    std::f64::consts::PI + 10 as f64
    }
    else {
    *x = 20;

    // also return value, but a bad practice
    return std::f64::consts::PI
    }

}