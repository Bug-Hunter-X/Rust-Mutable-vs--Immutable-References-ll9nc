fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    *y = 10; // Modify x through y
    println!("x = {}", x); // x is now 10
    let z = &x; // z is an immutable reference to x
    // *z = 20; // This line would cause a compile-time error because z is immutable
    println!("x = {}", x); // x is still 10
}