fn main() {
    /*
       A tuple is a collection of values of different types. Functions can use tuples to return multiple values, as tuples can hold any number of values.
    */

    // Create a tuple
    let person: (&str, i32, bool) = ("Alice", 25, true);

    // Access tuple elements
    let name = person.0; // Access the first element (name)
    let age = person.1; // Access the second element (age)
    let is_student = person.2; // Access the third element (is_student)

    // Print tuple elements
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is Student: {}", is_student);

    // Destructuring tuple
    let (name, age, is_student) = person;

    // Print destructured tuple elements
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is Student: {}", is_student);
}
