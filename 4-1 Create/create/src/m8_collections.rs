use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_hashmap() {

        let person_1:&str = "Alice";
        let person_2:&str = "Bob";
        let person_3:&str = "Charlie";

        //&str -> u32
        //key -> value

        let mut results_hm:HashMap<&str,u32> = HashMap::new();
        results_hm.insert(person_1, 55);
        results_hm.insert(person_2, 52);

        let testscore_hm:Option<&u32> = results_hm.get(person_3);
        dbg!(testscore_hm);

        if results_hm.contains_key("Alice") {
            dbg!("Alice is present!");
        }
    }

    #[test]
    fn tests_hashsets() {
        //Hashsets is a good way to manage data in-memory
        let mut names_hs = HashSet::new();
        names_hs.insert("Alice");
        names_hs.insert("Bob");
        names_hs.insert("Jane");

        if names_hs.contains("Bob") {
            dbg!("Bob is in here!");
        }
    }
}