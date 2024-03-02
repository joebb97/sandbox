fn main() {
    // Using Box::new with a string literal
    let my_string: Box<str> = "Hello, Box<str>!".into();

    // Print the string
    println!("{}", my_string);
}
