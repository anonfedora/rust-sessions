fn main() {
    // intro_to_u();
    string_handler();
}

// function that encapsulate all integers
fn intro_to_u() {
    // subtract
    // multiplication
    // division
    let sum_result: u8 = sum(5, 10);
    println!("the sum result is: {}", sum_result);
}

fn sum(x: u8, y: u8) -> u8 {
    x + y // implicit return
    //    return x + y; // explicit return
}

// handle all string-related functions
fn string_handler() {
    // intro_to_str_slice();
    intro_to_ownable_string();
}

// intro string slice
// for fixed-sized strings
fn intro_to_str_slice() {
    let name: &str = "Sylvia";
    println!("my name is {}", name)
}

fn intro_to_ownable_string() {
    let mut name: String = String::from("Wisdom");
    println!("first name: here: {}", name);
    name.push_str(" John");
    println!("final name: here: {}", name);
    println!("ptr = address in heap memory: {:?}", name.as_ptr());
}


