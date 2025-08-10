fn change_string(input:&mut String) {
    input.push('?');
}

fn main() {
    let mut s:String = String::from("Mutable reference here");
    let _t:&str = &s;
    s.push('?');
    println!("s is {}", s);
    //you can no longer print or use t because it is an immutable borrow!
    //println!("t is {}", t);

    let mut j:String = String::from("Family");
    let k:&mut String = &mut j;
    k.push('!');
    //println!("k is {}", k);
    change_string(k);
    println!("k is {}", k);
    println!("j is {}", j);
}
