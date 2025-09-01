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
    
    }
}