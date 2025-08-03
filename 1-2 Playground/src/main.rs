fn add_five(num:u32) -> u32 {
	let result:u32 = num + 5;
    return result
}

fn main() {
    println!("Hello, world!");
    let my_num:u32 = add_five(7);
    println!("My lucky number is {}", my_num);
}
