/*
   ENUMS
   Enums in Rust are a custom data type that allow you to define a type by enumerating its possible variants. Each variant represents a distinct value within the enum. Enums are used to represent a fixed set of related values or states.

   Type aliases
   Type aliases are Rust's ability to define a new type by aliasing an existing type. Type aliases are usually used to define commonly used types.
*/

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// Define an enum to classify web events.
// Each variant represents a different type of event.
enum WebEvent {
    PageLoad,                 // Represents a page load event
    PageUnload,               // Represents a page unload event
    KeyPress(char),           // Represents a key press event with the pressed character
    Paste(String),            // Represents a paste event with the pasted string
    Click { x: i64, y: i64 }, // Represents a click event with x and y coordinates
}

// A function that takes a WebEvent enum as an argument and prints information about the event.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::KeyPress(c) => println!("Key pressed: '{}'", c),
        WebEvent::Paste(s) => println!("Text pasted: \"{}\"", s),
        WebEvent::Click { x, y } => println!("Clicked at x={}, y={}", x, y),
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    // We can refer to each variant via its alias, not its long and inconvenient name
    let x = Operations::Add;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
