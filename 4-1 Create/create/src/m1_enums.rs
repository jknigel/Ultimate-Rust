#[derive(Debug)]
enum CarColour {
    Red,
    Green,
    Blue,
    Silver,
    Rainbow
}

#[derive(Debug)]
enum GivenResult<T,E> {
    Ok(T),
    Err(E)
}

#[derive(Debug)]
enum GivenOption<T> {
    None,
    Some(T)
}

fn create_car_colour_rainbow() -> CarColour {
    let my_car_colour:CarColour = CarColour::Rainbow;
    return my_car_colour
}

fn check_under_five(num_check:u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        return GivenResult::Ok(num_check)
    } else {
        return GivenResult::Err("Not under 5!".to_string())
    }
}

//This function uses built-in result, no need define enum
fn check_under_five_built_in(num_check:u8) -> Result<u8, String> {
    if num_check < 5 {
        return Ok(num_check)
    } else {
        return Err("Not under 5!".to_string())
    }
}

fn remainder_zero(num_check:f32) -> GivenOption<f32> {
    let remainder:f32 = num_check % 10.0;
    if remainder != 0.0 {
        return GivenOption::Some(remainder)
    } else {
        return GivenOption::None
    }
}

fn remainder_zero_built_in(num_check:f32) -> Option<f32> {
    let remainder:f32 = num_check % 10.0;
    if remainder != 0.0 {
        return Some(remainder)
    } else {
        return None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let car_colour:CarColour = create_car_colour_rainbow();
        dbg!(car_colour);

        let is_under_five_result:GivenResult<u8, String> = check_under_five(2);
        dbg!(is_under_five_result);

        let is_not_under_five_result:GivenResult<u8, String> = check_under_five(7);
        dbg!(is_not_under_five_result);

        let build_in_under_five_result:Result<u8, String> = check_under_five_built_in(3);
        dbg!(build_in_under_five_result);

        let remainder:GivenOption<f32> = remainder_zero(12.2);
        dbg!(remainder);

        let no_remainder:GivenOption<f32> = remainder_zero(10.0);
        dbg!(no_remainder);

        let remainder_built_in:Option<f32> = remainder_zero_built_in(18.8);
        dbg!(remainder_built_in);
    }
}