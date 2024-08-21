fn main() {
    // Imutable variable
    let x = 5;
    println!("O valor de x é: {}", x);
    // x = 6; Compiler error: cannot assign twice to immutable variable
    println!("O valor de x é: {}", x);

    // Mutable variable
    let mut y = 5;
    println!("O valor de y é: {}", y);
    y = 6;
    println!("O valor de y é: {}", y);

    // Constants
    const Z: i32 = 10; // Must have a type definition
    println!("O valor de Z é: {}", Z); // Should have a UPPER_CASE name

    // Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("O valor de x é: {}", x);
    let x = "Change type of x";
    println!("O valor de x é: {}", x);

    // Data Types
    let _i: u32 = 12; // Int Unsigned (i8, i16, i32, i64, isize)
    let _i: i32 = -12; // Int Signed (u8, u16, u32, u64, usize)
    let _i: f32 = 3.5; // Float 32bits
    let _i = 3.5; // Float 64bits
    let _i: bool = true; // Boolean
    let _i: char = 'A'; // Charactere
    let _i: &str = "A"; // String
    let _i: (i32, f64, bool) = (2, 3.4, false); // Tuple
    let _i = [1, 2, 3, 4, 5]; // Matrix (Imutable size)
    let _i = 0..20; // Range
    let _i: Vec<i32> = (0..20).collect(); // Vector
    // println!("O valor de i é: {}", i);
}
