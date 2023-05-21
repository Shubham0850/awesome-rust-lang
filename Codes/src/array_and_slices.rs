use std::mem;

fn main() {
    /*
       ARRAY
       An arrray is a collection of objects of the same type 'T', stored in contiguous memory. Arrays are created using brackets [], and their length, which is known at compile time, is part of their type signature [T; lenght].

       SLICES
       A slice is a reference to a section of an array. Slices are created using brackets [start..end], where start and end are indices into the array. It is similar to arrays, but their length is not known at compile time.

    */

    // Static Array
    let static_array: [i32; 5] = [1, 2, 3, 4, 5];
    // An array with a fixed length of 5 and elements of type i32

    // Accessing array elements
    let first_element = static_array[0];
    let second_element = static_array[1];
    // Access individual elements using indexing starting from 0

    // Modifying array elements
    let mut mutable_array = [10, 20, 30];
    mutable_array[1] = 25;
    // Modify elements of a mutable array

    // Array Slices
    let array_slice = &static_array[1..4];
    // Create a slice referencing elements 2, 3, and 4 of the static_array

    // Functions that accept slices
    print_slice(array_slice);
}

fn print_slice(slice: &[i32]) {
    // Function accepting a slice
    for element in slice {
        // Iterate over the elements of the slice
        println!("Element: {}", element);
    }
}
