fn main() {
    let s:String = String::from("Hello");
    let t:&String = &s;
    println!("t is {}", t); //Dereference Coercion

    let mut name:String = String::from("Johnny");
    let name_t:&mut String = &mut name;
    *name_t = String::from("Desmond");

    println!("Name_t is {}", name_t); //Must use name_t first, because can only have 1 reference and it is active here.
    println!("Name is {}", name);

    let mut x:i32 = 70;
    println!("x is {}", x);

    let y:&mut i32 = &mut x;
    *y += 1; // When using reference, you MUST dereference it in order to mutate it
    println!("y is {}", y);

    println!("x is {}", x);
    x = 50;
    println!("x is {} now", x);
}
