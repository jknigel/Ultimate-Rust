/// Function: add_five
/// # Arguments (num:u32)
/// 
/// # Returns u32
/// 
/// # Example
/// ```
/// let x = 5;
/// let y = add_five(x);
/// ```
/// # Output
/// y = 10
///
/**
 * This is a multiline block
 * For the function add_five
 */

//Add five function
pub fn add_five(num:u32) -> u32 {
	let result:u32 = num + 5;
    return result
}

//Multiply five function
pub fn multiply_five(num:u32) -> u32 {
	let result:u32 = num * 5;
    return result
}

//Testing
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