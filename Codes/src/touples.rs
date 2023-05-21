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

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    println!("Pair is {:?}", tuple_of_tuples);

    // But long Tuples ( more than 12 elements ) cannot be printed.
    // let too_long_tuple = (1, 3, 4, 5, 23, 12, 13, 12, 543, 23, 121, 334, 123);

    

    println!("Too long touples {:?}", too_long_tuple)
}
