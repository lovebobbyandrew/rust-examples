fn main() {
    // Without the mut keyword, this operation would be forbidden.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is now: {}", x);
}
