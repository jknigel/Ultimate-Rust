#[cfg(test)]
mod tests {
    //Initialize first declarative macro
    macro_rules! mad_skills {
        /* 
        ($x:expr) => {
            format!("You sent an expression: {}", $x)
        }
        */
        ($x:ty) => {
            match stringify!($x) {
                "i32" => "You sent an i32 type".to_string(),
                _ => "You sent something else".to_string(),
            }
        }
    }

    //Initialize vector declarative macro
    macro_rules! my_vec {
        ($($x: expr),+ ) => {
            {
                let mut temp_vec = Vec::new();

                $(
                    temp_vec.push($x);
                )+
                temp_vec
            }
        }
    }

    #[test]
    fn tests_declarative_macro() {
        println!("Hello 1");
        dbg!("Hello 2");

        let x:Vec<i32> = vec!(1,2,3);
        let formatted:String = format!("Hello 3 with vec {:?}", x);
        dbg!(formatted);
        println!("------");

        let some_var:String = mad_skills!(i32);
        dbg!(some_var);

        let mut y:Vec<i32> = my_vec!(1);

        dbg!(y);
    
    }
}