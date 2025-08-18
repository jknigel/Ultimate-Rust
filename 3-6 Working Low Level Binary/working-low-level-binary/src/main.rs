fn main() {
    //Binary
    let a:u8 = 0b_1010_1010;
    let b:u8 = 0b_0101_1010;
    println!("a's value is {}", a);
    println!("b's value is {}", b);

    //Show operations working in the wild
    println!("a in binary {:08b}", a);
    println!("b in binary {:08b}", b);
    
    //Logic Gates
    println!("AND {:08b}", a & b);
    println!("OR {:08b}", a | b);
    println!("XOR {:08b}", a ^ b);
    println!("NOT {:08b}", !a);

    //Bitwise Operations
    println!("a << 1 {:08b}", a << 1); //shift to the left
    println!("a << 1 {}", a << 1); 
    println!("a >> 1 {:08b}", a >> 1);
    println!("a >> 1 {}", a >> 1);

    //Little Endian
    let n:u16 = 0x1234;
    println!("n is: {:?}", 0x1234);

     let big_endian:[u8; 2] = n.to_be_bytes();
     let little_endian:[u8; 2] = n.to_le_bytes();

     println!("n in big endian: {:02X}-{:02X}", big_endian[0], big_endian[1]);
     println!("n in little endian: {:02X}-{:02X}", little_endian[0], little_endian[1]);
}
