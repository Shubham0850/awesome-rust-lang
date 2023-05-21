fn main() {
    // Arithmetic Operators

    let a = 10;
    let b = 5;

    // Addition
    let addition = a + b;  // 10 + 5 = 15

    // Subtraction
    let subtraction = a - b;  // 10 - 5 = 5

    // Multiplication
    let multiplication = a * b;  // 10 * 5 = 50

    // Division
    let division = a / b;  // 10 / 5 = 2

    // Remainder (Modulo)
    let remainder = a % b;  // 10 % 5 = 0

    // Comparison Operators

    // Equal to
    let equal_to = a == b;  // false

    // Not equal to
    let not_equal_to = a != b;  // true

    // Greater than
    let greater_than = a > b;  // true

    // Less than
    let less_than = a < b;  // false

    // Greater than or equal to
    let greater_than_or_equal_to = a >= b;  // true

    // Less than or equal to
    let less_than_or_equal_to = a <= b;  // false

    // Logical Operators

    let p = true;
    let q = false;

    // Logical AND
    let logical_and = p && q;  // false

    // Logical OR
    let logical_or = p || q;  // true

    // Logical NOT
    let logical_not = !p;  // false

    // Assignment Operators

    let mut x = 5;

    // Addition assignment
    x += 2;  // Equivalent to: x = x + 2; (x = 5 + 2)

    // Subtraction assignment
    x -= 3;  // Equivalent to: x = x - 3; (x = 7 - 3)

    // Multiplication assignment
    x *= 4;  // Equivalent to: x = x * 4; (x = 4 * 4)

    // Division assignment
    x /= 2;  // Equivalent to: x = x / 2; (x = 16 / 2)

    // Remainder assignment
    x %= 5;  // Equivalent to: x = x % 5; (x = 8 % 5)

    // Print the results
    println!("Arithmetic Operators: {} {} {} {} {}", addition, subtraction, multiplication, division, remainder);
    println!("Comparison Operators: {} {} {} {} {} {}", equal_to, not_equal_to, greater_than, less_than, greater_than_or_equal_to, less_than_or_equal_to);
    println!("Logical Operators: {} {} {}", logical_and, logical_or, logical_not);
    println!("Assignment Operators: {}", x);
}
