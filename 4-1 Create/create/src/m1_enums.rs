#[derive(Debug)]
enum CarColour {
    Red,
    Green,
    Blue,
    Silver,
    Rainbow
}

fn create_car_colour_rainbow() -> CarColour {
    let my_car_colour:CarColour = CarColour::Rainbow;
    return my_car_colour
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let car_colour:CarColour = create_car_colour_rainbow();
        dbg!(car_colour);
    }
}