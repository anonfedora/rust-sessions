fn main() {
    intro_to_u();
}

// function to encapsulate all integers
fn intro_to_u(){
    let sum_result: u8 = sum(5, 10);
    let mult_result: u64 = multiply(5, 10);
    let divide: f64 = divide(20.0, 10.2);
    let subtr: isize = substract(20, 10);
    let check: bool = check_func(5, 10);
    println!("Sum: {}", sum_result);
    println!("Multiplication: {}", mult_result);
    println!("Division: {}", divide);
    println!("Substraction: {}", subtr);    
    println!("Check: {}", check);

    let sum_result: f64 = sumfp(5.0, 10.0);
    let mult_result: f64 = multiplyfp(5.0, 10.0);
    let divide: f64 = dividefp(20.0, 10.2);
    let subtr: f64 = substractfp(20.0, 10.0);
    println!("Sum: {}", sum_result);
    println!("Multiplication: {}", mult_result);
    println!("Division: {}", divide);
    println!("Substraction: {}", subtr);

    let full_name = string_formatting("Akinshola", "Akinniyi");
    println!("Full Name: {}", full_name);
}

fn sum(x: u8, y: u8) -> u8 {
    x + y
}
fn multiply(x: u64, y: u64) -> u64 {
    x * y
}
fn divide(x: f64, y: f64) -> f64 {
    let res: f64 = x / y;
    return res
}
fn substract(x: isize, y: isize) -> isize {
    x - y
}

fn sumfp(x: f64, y: f64) -> f64 {
    x + y
}

fn multiplyfp(x: f64, y: f64) -> f64 {
    x * y
}

fn dividefp(x: f64, y: f64) -> f64 {
    x / y
}

fn substractfp(x: f64, y: f64) -> f64 {
    x - y
}

fn check_func(num1: u8, num2: u8) -> bool {
    let sum_of_two_nums = sum(num1, num2);
    if sum_of_two_nums % 2 == 0 {
        println!("The sum of {} and {} is even", num1, num2);
        return true;
    } else {
        println!("The sum of {} and {} is odd", num1, num2);
        return false;
    }
}

fn string_formatting(first_name: &str, last_name: &str) -> String {
    let full_name = format!("{} {}", first_name, last_name);
    return full_name;

}



// subtract
// multiplication
// division


