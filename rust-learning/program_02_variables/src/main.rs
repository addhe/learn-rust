fn main() {
    // === 1. IMMUTABLE VARIABLES ===
    // By default, variables in Rust are immutable, meaning their value cannot be changed.
    // We use the `let` keyword to declare them.
    let x = 5;
    println!("1. The value of immutable x is: {}", x);
    // The line below would cause a compile-time error:
    // x = 6; // error[E0384]: cannot assign twice to immutable variable `x`

    // === 2. MUTABLE VARIABLES ===
    // To create a variable whose value can be changed, we use the `mut` keyword.
    let mut y = 10;
    println!("2. The initial value of mutable y is: {}", y);
    y = 15;
    println!("   The new value of y is: {}", y);

    // === 3. SHADOWING ===
    // You can declare a new variable with the same name as a previous variable.
    // This is called "shadowing". It's different from a mutable variable because
    // we are creating a *new* variable. This is useful for changing the type of a value.
    let z = 20;
    println!("3. The value of z is: {}", z);
    let z = "twenty"; // Shadowing z with a new variable of a different type (string slice).
    println!("   The value of shadowed z is now: {}", z);

    // === 4. PRIMITIVE DATA TYPES ===
    // Rust is a statically typed language, which means it must know the types of all
    // variables at compile time. However, the compiler can usually infer the type.
    // We can also explicitly add a type annotation.

    // Integer Types (e.g., i8, u8, i32, u32, i64, u64, isize, usize)
    // `i` for signed (can be negative), `u` for unsigned (only positive).
    // The number is the number of bits. `i32` is the default.
    let an_integer: i32 = -500;
    println!("4. An integer: {}", an_integer);

    // Floating-Point Types (f32, f64)
    // `f64` is the default and has better precision.
    let a_float: f64 = 3.14;
    println!("   A float: {}", a_float);

    // Boolean Type (bool)
    let is_rust_fun: bool = true;
    println!("   Is Rust fun? {}", is_rust_fun);

    // Character Type (char)
    // Represents a single Unicode Scalar Value. Note the single quotes.
    let a_char: char = '🦀'; // Crabs are the Rust mascot!
    println!("   A character: {}", a_char);

    println!("\nVariables and data types exploration complete!");
}
