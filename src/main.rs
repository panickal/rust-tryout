// hello.rs
fn main() {
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
