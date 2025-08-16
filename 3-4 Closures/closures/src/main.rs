fn main() {
    let num:i32 = 5;
    let add_num = |x:i32| x + num;
    let new_num = add_num(8);
    println!("Num is {}", new_num);
}
