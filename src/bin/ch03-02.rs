// Rust is a statically typed language
fn main() {
    /* Scalar Types */
    println!("\n:::Scalar Types:::");

    /* Integer Literals */
    println!("\n::Integer Literals::\n");

    // i32 is the fastest and the default
    let decimal = 98_222;
    println!("Decimal number: {}", decimal);

    let hex = 0xff;
    println!("Hexadecimal number: {}", hex);

    let octal = 0o77;
    println!("Octal number: {}", octal);

    let binary = 0b1111_0000;
    println!("Binary number: {}", binary);

    // u8 only
    let byte = b'A';
    println!("Byte number: {}", byte);

    /* Floating-Point Types */
    println!("\n::Floating-Point Types::\n");

    let y: f32 = 3.0;
    println!("f32 number: {}", y);

    // f64 is the default
    let x = 2.0;
    println!("f64 number: {}", x);

    /* Numeric Operations */
    println!("\n::Numeric Operations::\n");

    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("4 * 30 = {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {}", remainder);

    /* The Boolean Type */
    println!("\n::The Boolean Type::\n");

    let t = true;
    println!("true: {}", t);

    let f: bool = false; // with explicit annotation
    println!("false: {}", f);

    /* The Character Type */
    println!("\n::The Character Type::\n");

    let c = 'z';
    println!("char: {}", c);

    let z = 'ℤ';
    println!("char: {}", z);

    let heart_eyed_cat = '😻';
    println!("char: {}", heart_eyed_cat);

    /* Compound Types */
    println!("\n:::Compound Types:::");

    /* The Tuple Type */
    println!("\n::The Tuple Type::\n");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x from tup is: {}", x);
    println!("The value of y from tup is: {}", y);
    println!("The value of z from tup is: {}", z);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    println!("The value of x.0 from tuple x is: {}", five_hundred);

    let six_point_four = x.1;
    println!("The value of x.1 from tuple x is: {}", six_point_four);

    let one = x.2;
    println!("The value of x.2 from tuple x is: {}", one);

    /* The Array Type */
    println!("\n::The Array Type::\n");

    let a = [1, 2, 3, 4, 5];
    println!("Array 0: {:?}", a);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array 1: {:?}", a);

    let a = [3; 5];
    println!("Array 2: {:?}", a);

    let first = a[0];
    let second = a[1];
    println!(
        "The first element is {}, and the second is {}",
        first, second
    );

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("Array months: {:#?}", months);
}
