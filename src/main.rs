//mod strings;
mod constructor;
mod float;
mod signed;
mod string;
mod unsigned;
mod user_struct;

fn main() {
    unsigned::intro_to_u();
    signed::intro_to_i();
    float::intro_to_float();
    string::strings();
    user_struct::user_registry();

    //Book creation
    let book = constructor::Book::new("The Rust Programming Language", "Steve Klabnik", 2019);

    println!("Title: {}", book.title);
    println!("Author: {}", book.author);
    println!("Year: {}", book.year);
    println!("Likes: {}", book.likes);
}
