enum Message {
    Quit,
    ChangeColour(i32,i32,i32),
    Move {x:i32, y:i32},
    Write(String)
}

fn process_message(msg:Message) {
    match msg {
        Message::Quit => {
            println!("I quit!");
        },
        Message::ChangeColour(red, blue, green) => {
            println!("Red {}, Blue {}, Green {}", red, blue, green);
        },
        Message::Move {x, y:new_name} => {
            println!("X is {}, Y as new_name is {}", x, new_name);
        },
        Message::Write(text) => {
            println!("{}", text);
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_large_enum() {
        let my_quit:Message = Message::Quit;
        process_message(my_quit);

        let my_colour:Message = Message::ChangeColour(10,50,88);
        process_message(my_colour);

        let my_move:Message = Message::Move{x:88, y:188};
        process_message(my_move);

        let my_write:Message = Message::Write("My awesome Rust!".to_string());
        process_message(my_write);
    }

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

    #[test]
    fn tests_match_guard() {
        let pair:(i32,i32) = (2,-2);
        match pair {
            (x,y) if x==y => println!("They match!"),
            (x,y) if x+y==0 => println!("They neutralize"),
            (_,y) if y==2 =>println!("Y is indeed +2"),
            _ => println!("We don't care!")
        };
    }

    #[test]
    fn tests_match_struct() {
        struct Location {
            x:i32,
            y:i32
        }

        let my_location:Location = Location{x:0,y:128};

        match my_location {
            Location{x,y:0} => println!("Y is on the axis"),
            Location{x:0,y} => println!("X is on the axis"),
            Location{x,y} => println!("X and Y are not on the axis"),
        };
    }
}