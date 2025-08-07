const MSG_CONST:&str = "Hello World!";

fn main() {
    //Heap
    let my_string:String = String::from("Jknigel360 Hello!");
    let s_2:&str = &my_string[0..10];
    println!("{}", s_2);

    //Stack - This is String Literal. Stored statically, read-only memory
    let msg:&str = "Jknigel888";
    println!("{}", msg);

    //Heap
    let msg_string:String = "Jknigel 365".to_string();
    println!("{}", msg_string);

    //Stack
    println!("{}", MSG_CONST);
}
