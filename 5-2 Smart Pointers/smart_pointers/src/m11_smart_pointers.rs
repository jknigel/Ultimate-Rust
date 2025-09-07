#[cfg(test)]

mod test {
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_box_smart_pointers() {
        #[derive(Debug)]
        struct Node {
            id:u32,
            next:Option<Box<Node>> //This is to link to next node, Box allocates memory. Use option to handle the end where no node exists.
        }

        let nodes:Box<Node> = Box::new(
            Node {id: 0, next: Some(
                Box::new(Node {id: 1, next: Some(
                    Box::new(
                        Node {id: 2, next: None}
                    )
                )})
            )}
        );
        dbg!(nodes);

    }

    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_reference_counter() {
        
        let mut x:Rc<i32> = Rc::new(50); //Rc is reference counter: it will count number of references to it
        let y:Rc<i32> = Rc::clone(&x);

        x = Rc::new(70);

        dbg!(y);
        dbg!(x);

        let a:Rc<RefCell<i32>> = Rc::new(RefCell::new(80)); //This allows us to mutate all the values that share reference with variable a
        let b:Rc<RefCell<i32>> = Rc::clone(&a);

        *a.borrow_mut() = 100; //can use *b.borrow_mut() as well
        dbg!(a);
        dbg!(b);

    }
}