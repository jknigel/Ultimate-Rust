fn main() {
    // Number Literals
    println!("Big Number is {}", 98_222_000);
    println!("Hex is {}", 0xff);
    println!("Octal is {}", 0o77);
    println!("Binary is {}", 0b1111_0000);
    println!("Bytes 'A' is {}", b'A');
    println!("Bytes 'Z' is {}", b'Z');

    //Raw - String Literal
    let text:&str = r#"{"message" : "Rust is Awesome!"}"#;
    println!("Raw-String literal is {}", text);
}
