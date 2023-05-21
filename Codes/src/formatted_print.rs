fn main() {
    // prints the number of days -> {} will be replace by the argument 31
    println!("{} days", 31);

    // Prints a greeting message with positional arguments
    println!("{0} is {1}. {1} is {0}", "Shubham", "Reach");

    // Prints a sentence using named arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fix",
        verb = "jumps over"
    );

    // Prints numbers in different bases: decimal, binary, octal, and hexadecimal
    println!("Base 10: {}", 9999);
    println!("Base 2 (binary): {:b}", 9999);
    println!("Base 8 (octal): {:o}", 9999);
    println!("Base 16 (hexadecimal): {:x}", 9999);

    // You can right-justify text with a specified width.
    // This will output: "    1".
    // Four white spaces and a "1", for a total width of 5.
    println!("{number:>5}", number = 1);

    // You can pad numbers with extra zeroes,
    // and left-adjust by flipping the sign.
    // This will output "10000".
    println!("{number:0<5}", number = 1);

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number = 1, width = 5);

}
