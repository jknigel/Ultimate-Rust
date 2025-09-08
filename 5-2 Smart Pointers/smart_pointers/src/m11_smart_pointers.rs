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
        dbg!(&a);//&a is used if we want to still use a because dbg takes ownership of the variable.
        dbg!(&b);

        *b.borrow_mut() = 100; //can use *b.borrow_mut() as well
        dbg!(&a);
        dbg!(&b);
    }

    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_smart_pointers_weak() {
        #[derive(Debug)]
        struct House {
            address_number:u16,
            street:String,
            furniture:RefCell<Vec<Rc<Furniture>>>
        }

        #[derive(Debug)]
        struct Furniture {
            id:String,
            description:String,
            house:Weak<House> //A weak pointer can reference an object without owning it
        }

        let house1:Rc<House> = Rc::new(House {
            address_number: 123,
            street: "coding avenue".to_string(),
            furniture: RefCell::new(vec!())
        });

        let table:Rc<Furniture> = Rc::new(Furniture {
            id: "Table1".to_string(),
            description: "Kitchen Table".to_string(),
            house: Rc::downgrade(&house1) //Ownership has shifted here, so house1 cannot be used elsewhere
        });

        let desk:Rc<Furniture> = Rc::new(Furniture {
            id: "Desk1".to_string(),
            description: "Office Desk".to_string(),
            house: Rc::downgrade(&house1)
        });

        house1.furniture.borrow_mut().push(Rc::clone(&table));
        house1.furniture.borrow_mut().push(Rc::clone(&desk));

        dbg!(house1);
    }

    
}