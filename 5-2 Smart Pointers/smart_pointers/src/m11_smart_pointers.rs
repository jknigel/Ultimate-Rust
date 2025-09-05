#[cfg(test)]

mod test {
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
}