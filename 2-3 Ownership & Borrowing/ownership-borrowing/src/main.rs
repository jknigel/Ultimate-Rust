fn make_string_not_dangle() -> String {
    let j:String = String::from("Hub360+ is no.1 app in the universe");
    //let k:&str = &j;
    return j
}

fn main() {
    //Works
    let x:i32 = 50;
    let y:i32 = x;
    println!("x is {}", x);
    println!("y is {}", y);

    //Works but double memory
    let s:String = String::from("Jknigel is the worst");
    let t:String = s.clone();
    println!("s is {}", s);
    println!("t is {}", t);

    //Works
    let a:String = String::from("Jknigel is the best");
    let b:&str = &a;
    println!("a is {}", a);
    println!("b is {}", b);

    //Cannot call dangling variable/function because it is a stack frame and it will go out of scope
    let r:String = make_string_not_dangle();
    println!("r is {}", r);
}
