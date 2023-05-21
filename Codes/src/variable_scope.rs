/*
   Variable scope in Rust refers to the portion of the code where a variable is valid and can be accessed. Rust enforces strict rules around variable scope to ensure memory safety and prevent the use of invalid or uninitialized data.
*/

fn main() {
    // Variable `x` is declared and bound to the value 42.
    let x = 42;

    // The variable `x` is valid within this scope.
    println!("x: {}", x);

    // Variable `y` is declared and bound to the value 10.
    let y = 10;

    // A new block scope is introduced.
    {
        // The variable `y` is also valid within this inner scope.
        println!("y: {}", y);

        // Variable `z` is declared and bound to the value 5.
        let z = 5;

        // The variable `z` is valid within this inner scope.
        println!("z: {}", z);
    } // The inner scope ends here, and variable `z` goes out of scope.

    // Variable `z` is no longer valid at this point and cannot be accessed.

    // Attempting to use `z` here would result in a compilation error.

    // The variable `y` is still valid at this point.
    println!("y: {}", y);
} // The main function ends here, and variable `x` and `y` go out of scope.
