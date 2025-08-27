#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_match_literals() {
        let number: i32 = 20;

        let res:&str = match number {
            1 => "This is the first!",
            2 | 3 | 5 | 6 | 15 | 20 => "This is the lottery numbers!",
            _ => "You did not win anything."
        };

        println!("{}", res);
    }

    #[test]
    fn tests_match_option() {
        let some_num:Option<i32> = Some(10);
        let prob_none:Option<i32> = None;

        let res2 = match some_num {
            Some(i) => i,
            None => {
                panic!("There was a problem!");
            }
        };

        println!("{:?}", some_num);
        println!("{}", res2);
        
        //This code will panic.
        /*
        let res3 = match prob_none {
            Some(i) => i,
            None => {
                panic!("There was a problem!");
            }
        };

        println!("{:?}", prob_none);
        println!("{}", res3);
        */

        if let Some(i) = prob_none {
            println!("{}", i);
        } else {
            println!("100");
        }

        let my_int:i32 = if let Some(i) = some_num {
            i
        } else {
            panic!("There is a problem!");
        };
        println!("My int = {}", my_int);
    }

    #[test]
    fn tests_match_result() {
        let some_res:Result<i32, &str> = Ok(50);
        let some_err:Result<i32, &str> = Err("There is a BIG problem!");

        let res = match some_res {
            Ok(val) => val,
            Err(e) => panic!("{}", e)
        };
        println!("{}", res);

        //This code will panic due to error
        /*
        let res2 = match some_err {
            Ok(val) => val,
            Err(e) => panic!("{}", e)
        };
        println!("{}", res);
        */
        
        let my_int:i32 = if let Ok(i) = some_res {
            i
        } else {
            panic!("There is a BIG problem!");
        };
        println!("My int = {}", my_int);
    }
}