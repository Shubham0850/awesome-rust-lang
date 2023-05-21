
/*
	Rust has tow different types of constants which can be declared in any scope including global. Both require explicit type annotation:

	'const': An unchangeable value 
	'static': A possibly 'mut' able variable with 'static' lifetime. The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is 'unsafe'.
 */

// Define a constant named PI with a value of 3.14159
const PI: f32 = 3.14159;
static mut GLOBAL_VALUE: i32 = 42;

fn main() {
    // Print the value of the constant PI
    println!("The value of PI is: {}", PI);
	
    // Use the constant in a calculation
    let radius = 5.0;
    let circumference = 2.0 * PI * radius;
    println!("The circumference of a circle with radius {} is: {}", radius, circumference);

	// Update the value of the static variable
	unsafe{
		// Modifying a mutable static variable requires an 'unsafe' block
		GLOBAL_VALUE = 50;
		println!("The updated value of GLOBAL_VALUE is: {}", GLOBAL_VALUE)
	}
}
