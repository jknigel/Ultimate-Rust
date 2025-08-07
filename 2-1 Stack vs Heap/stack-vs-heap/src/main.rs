const MY_INTEGER:u8 = 10;

fn main() {
    //Stack
    let x:u8 = 50;
    println!("x is {}", x);

    //Heap
    let mut arr:Vec<u8> = vec![1,2,3,4,5];
    arr.push(6);
    println!("Vec is {:?}", arr);

    //Referencing the Stack pointing to a value on the Heap (arr)
    let arr_2:&[u8] = &arr[0..3];
    println!("arr_2 is {:?}", arr_2);

    //Heap because growable & shrinkable size
    let mut s:String = String::from("Jknigel360");
    s.push(' ');
    s.push('!');
    println!("s is {}", s);

    //A Reference on the Stack pointing to a value on the Heap
    let s_2:&str = &s[0..5];
    println!("s_2 is {:?}", s_2);

    //Global variable
    println!("My integer is {}", MY_INTEGER);
}
