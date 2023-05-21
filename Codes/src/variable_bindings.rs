/*
   In Rust, variable binding refers to the process of introducing a new variable and associating it with a particular value or expression. Rust provides several ways to bind variables, including using the 'let' keyword and pattern matching.
*/

fn main() {
    let an_integer = 1u32; // variable an_integer is bound to the value 1
    let a_boolean = true; // variable a_boolean is bound to the value true
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning
    // Please note that warnings may not be shown in a browser
}
