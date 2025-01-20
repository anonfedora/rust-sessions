fn main() {
    intro_to_u();
    println!("------------string handler-------------");
    string_handler();
    println!("------------number data type conversion-------------");
    nunber_data_type_converter();
    println!("------------string data type conversion-------------");
    string_data_type_converter();
    println!("------------unsigned number-------------");
    intro_to_i();
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

    let sum_result: f64 = sum_fp(5.0, 10.0);
    let mult_result: f64 = multiply_fp(5.0, 10.0);
    let divide: f64 = divide_fp(20.0, 10.2);
    let subtr: f64 = substract_fp(20.0, 10.0);
    println!("Sum: {}", sum_result);
    println!("Multiplication: {}", mult_result);
    println!("Division: {}", divide);
    println!("Substraction: {}", subtr);

    let full_name = string_formatting(convert_to_string_v1("Abdulmajid"), convert_to_string_v2("Yunus"));
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

fn sum_fp(x: f64, y: f64) -> f64 {
    x + y
}

fn multiply_fp(x: f64, y: f64) -> f64 {
    x * y
}

fn divide_fp(x: f64, y: f64) -> f64 {
    x / y
}

fn substract_fp(x: f64, y: f64) -> f64 {
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

// fn string_formatting(first_name: &str, last_name: &str) -> String {
//     let full_name = format!("{} {}", first_name, last_name);
//     return full_name;
// }
fn string_formatting(first_name: String, last_name: String) -> String {
    let full_name = format!("{} {}", first_name, last_name);
    return full_name;
}

// util fn version 1 to convert &str to String 
fn convert_to_string_v1(x: &str) -> String {
    x.to_string()
}

// util fn version 2 to convert &str to String 
fn convert_to_string_v2(x: &str) -> String {
   String::from(x)
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



// function to encapsulate all integers converter
fn nunber_data_type_converter(){
    let low_to_high = low_to_high_nunber_data_type_converter(3);
    let high_to_low = high_to_low_nunber_data_type_converter(40);
    println!("A u8 signed  integer of {} is converted to u32 signed integer",low_to_high);
    println!("A u8 signed  integer of {} is converted to u32 signed integer",high_to_low);
}

// handle all u8 convertion to u32
fn low_to_high_nunber_data_type_converter(x:u8)->u32{
    x as u32 
}
// handle all u8 convertion to u32
fn high_to_low_nunber_data_type_converter(x:u32)->u8{
    x as u8 
}


// function to encapsulate all string fn
fn string_data_type_converter(){
    let string = String::from("fan of a fan");
    let string_to_str = string_to_str_string_data_type_converter(&string);
    println!("A String data type of {} is converted to str",string_to_str);
    //shadow
     let string = "it another beautifull day to see shege banza";
     let string_to_str = str_to_string_string_data_type_converter(string);
     println!("A str data type of {} is converted to string",string_to_str);
}

// handle all String convertion to str
fn string_to_str_string_data_type_converter(x:&String)->&str{
    x.as_str() 
}

fn str_to_string_string_data_type_converter(x:&str)->String{
    String::from(x) 
}

// function to encapsulate all integers
fn intro_to_i(){
    let sum_result:i8 = sum_unsign(5, 10);
    let mult_result:i64  = multiply_unsign(5, 10);
    let divide:i64 = divide_unsign(20, 10);
    let subtr:i8 = substract_unsign(20, 10);
    
    println!("Sum: {}", sum_result);
    println!("Multiplication: {}", mult_result);
    println!("Division: {}", divide);
    println!("Substraction: {}", subtr);    
}

fn sum_unsign(x: i8, y: i8) -> i8 {
    x + y
}
fn multiply_unsign(x: i64, y: i64) -> i64 {
    x * y
}
fn divide_unsign(x: i64, y: i64) -> i64 {
    let res: i64 = x / y;
    return res
}
fn substract_unsign(x: i8, y: i8) -> i8 {
    x - y
}

