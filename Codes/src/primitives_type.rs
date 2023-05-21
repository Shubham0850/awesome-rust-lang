fn main() {
	/*
		Rust provides access to a wide variety of primitives.

		Scalar Types:
		- Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
		- Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
		- Floating point numbers: f32, f64
		- Boolean: bool either true/false
		- char Unicode scalar values like 'a', '😃' ( 4 bytes each )
		- The unit type (), whose only possible value is an empty tuple: ()

		Compound Types
		- Arrays like [1, 2, 3]
		- Tuples like (1, true)
	 */
    // Scalar Types

    // Signed integers: Range of negative and positive whole numbers
    let age: i32 = 30;  // Signed 32-bit integer
    let count: i64 = -100;  // Signed 64-bit integer

    // Unsigned integers: Range of positive whole numbers
    let quantity: u16 = 42;  // Unsigned 16-bit integer
    let total: u64 = 100_000;  // Unsigned 64-bit integer

    // Floating-point types: Decimal numbers with fractional parts
    let pi: f32 = 3.14;  // 32-bit floating-point number
    let gravity: f64 = 9.81;  // 64-bit floating-point number

    // Unicode character type: Represents individual Unicode scalar values
    let heart_emoji: char = '❤';  // Unicode character '❤'

    // Boolean type: Represents true or false values
    let is_rust_fun: bool = true;  // Boolean: true or false

    // Compound Types

    // Array type: Fixed-size collection of elements of the same type
    let numbers: [i32; 3] = [1, 2, 3];  // Fixed-size array of integers

    // Tuple type: Heterogeneous collection of elements of different types
    let person: (&str, u8, bool) = ("Alice", 25, true);  // Tuple with mixed types
}
