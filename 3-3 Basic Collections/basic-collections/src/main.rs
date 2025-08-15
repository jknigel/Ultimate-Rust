fn main() {
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

    chars.iter().for_each(|c:&char| print!("{}", c));

    let chars_again:Vec<char> = vec!('h','e','l','l','o');
    println!("{:?}", chars_again);

    let collected:String = chars_again.iter().collect();
    dbg!(collected);

    for c in chars_again {
        print!("{}",c);
        if c == 'o' {
            println!(", world!");
        }
    }
}
