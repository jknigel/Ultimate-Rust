pub fn minus_five(num:u32) -> u32 {
	let result:u32 = num - 5;
    return result
}

pub fn divide_five(num:u32) -> u32 {
	let result:u32 = num / 5;
    return result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn divide_five_test() {
        let x:u32 = 100;
        let y:u32 = divide_five(x);
        println!("Test: {}", y);
        assert_eq!(y, 21);
    }
}