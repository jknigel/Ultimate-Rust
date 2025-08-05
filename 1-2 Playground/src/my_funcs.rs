pub fn add_five(num:u32) -> u32 {
	let result:u32 = num + 5;
    return result
}

pub fn multiply_five(num:u32) -> u32 {
	let result:u32 = num * 5;
    return result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_five_test() {
        let x:u32 = 100;
        let y:u32 = add_five(x);
        println!("Test: {}", y);
        assert_eq!(y, 105);
    }
}