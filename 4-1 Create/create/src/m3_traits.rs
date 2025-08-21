trait Attacker {
    fn choose_style(&self) -> String;
}

#[derive(Debug)]
enum Character {
    Warrior,
    Archer,
    Wizard
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Warrior => "Heavy Armour".to_string(),
            Character::Archer => "Medium Armour".to_string(),
            Character::Wizard => "Light Armour".to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_traits() {
        let my_character:Character = Character::Warrior;
        let chosen_style = my_character.choose_style();
        dbg!(my_character);
        dbg!(chosen_style);
    }
}