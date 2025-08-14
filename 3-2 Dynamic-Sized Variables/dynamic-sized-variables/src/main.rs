fn main() {
    let name:&str = "Jknigel";
    println!("Name is {}", name);

    let dynamic_name:String = String::from("Jknigel360");
    println!("Dynamic name is {}", dynamic_name);
    println!("Dynamic name is stored in memory {:p}", &dynamic_name);

    let another_dynamic_name:String = name.to_string();
    println!("Another dynamic name is {}", another_dynamic_name);
    println!("Another dynamic name is stored in memory {:p}", &another_dynamic_name);

    let str_slice:&str = &dynamic_name[0..5];
    println!("String slice is {}", str_slice);

    let mut chars:Vec<char> = Vec::new();
    chars.insert(0, 'z');
    chars.insert(1, 'y');
    chars.insert(2, 'x');
    chars.push('a');
    chars.push('b');
    chars.push('c');
    println!("Chars is {:?}", chars);

    let removed_char:char = chars.pop().unwrap();
    println!("Removed_char is {:?}", removed_char);
    println!("New chars is {:?}", chars);
}
