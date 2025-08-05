mod my_funcs;
mod other_funcs;
use crate::my_funcs::{add_five, multiply_five};
use crate::other_funcs::other_funcs1::{minus_five, divide_five};

fn main() {
    println!("Hello, world!");
    let my_num_add:u32 = add_five(7);
    println!("My lucky number is {}", my_num_add);
    println!("My unlucky number is {}", multiply_five(9));
    println!("My favourite number is {}", minus_five(8));
    println!("My disliked number is {}", divide_five(45));
}
