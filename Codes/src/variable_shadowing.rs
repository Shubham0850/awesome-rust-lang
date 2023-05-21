/*
   Variable shadowing in Rust occurs when a new variable with the same name is declared within an inner scope, effectively "shadowing" the outer variable with the same name. This allows you to rebind while keeping the original variable inact.
*/

fn main() {
    let x = 42;

    println!("Outer x: {}", x); // Output: Outer x: 42

    {
        let x = 10; // The variable `x` is shadowed within this inner scope.

        println!("Inner x: {}", x); // Output: Inner x: 10
    }

    println!("Outer x: {}", x); // Output: Outer x: 42
}
