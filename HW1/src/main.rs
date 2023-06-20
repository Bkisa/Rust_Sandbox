fn main() {
    // Create a new string
    let mut my_string = String::from("Hello, Rust!");

    // Pass the ownership of the string to a function
    take_ownership(my_string);

    // Error: Trying to use `my_string` after it has been moved
    println!("After function call: {}", my_string);

    // Create a new string
    let mut my_other_string = String::from("Hello again!");

    // Pass a mutable reference to the string to a function
    change_reference(&mut my_other_string);

    // Now we can use `my_other_string` after the function call
    println!("After reference change: {}", my_other_string);
}

// Function takes ownership of the string and does nothing with it
fn take_ownership(some_string: String) {
    // String is no longer valid here as it has been moved
}

// Function takes a mutable reference to a string and changes it
fn change_reference(some_string: &mut String) {
    some_string.push_str(" Rust is awesome!");
}
