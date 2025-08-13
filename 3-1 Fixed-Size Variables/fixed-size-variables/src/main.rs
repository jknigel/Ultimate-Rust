const OUR_COURSE:&str = "Rust with AutoGPT";

fn main() {
    println!("Welcome to this course on {}", OUR_COURSE);

    //Stack
    let x:i32;
    x = 2;
    println!("x is {}", x);

    let y:i32 = 4;
    println!("y is {}", y);

    // For loop
    for i in 0..=y {
        if i != 4{
            print!("{}, ", i);
        }
        else {
            println!("{}", i);
        }
    }

    let mut z:i32 = 5;
    print!("z was {} ", z);
    z=10;
    println!("but is now {}", z);

    let freezing_temp:f64 = -2.4;
    println!("Freezing temperature is {}", freezing_temp);

    let is_zero_remainder:bool = 10 % 4 != 0;
    println!("is_zero_remainder is {}", is_zero_remainder);

    let my_char:char = 'j';
    println!("my_char is {}", my_char);

    let emoji_char:char = '😍';
    println!("my i_char is {}", emoji_char);

    let my_ints:[f32;10] = [0.0; 10];
    println!("my_ints are {:?}", my_ints);

    let my_floats_new:[f32;10]=my_ints.map(|n:f32| n + 1.8); 
    println!("my_floats_new is {:?}", my_floats_new);
}
