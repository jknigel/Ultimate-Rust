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
    }
}