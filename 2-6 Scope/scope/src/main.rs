fn main() {
    let mut c:i32 = 0;
    for i in 0..10 {
        // let mut c:i32 = 0; Cannot do this because it will go out of scope after the for loop
        if i % 2 == 0 {
            c += i;
        }
    }
    println!("c is {}", c);
}
