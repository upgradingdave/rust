pub struct Node {
    pub elem: usize,
    pub next: Option<Box<Node>>,
}

impl Node {
    pub fn new(elem: usize) -> Self {
        Node { elem: elem, next: None}
    }
    
    pub fn gen_cycle(tail_size: usize, loop_size: usize) -> Node {
        let mut idx = 0;
        let mut head = Node::new(idx);
        idx = idx + 1;

        let mut tail = &mut head;
        
        for i in idx..tail_size {
            let node = Node::new(i);
            tail.next = Some(Box::new(node));
            tail = tail.next.as_mut().unwrap();
        }
        
        idx = tail_size;

        if loop_size > 0 {
            
            //let loop_start = tail as *mut Node;
            let loop_start = Node::new(idx);
            tail.next = Some(Box::new(loop_start));
            tail = tail.next.as_mut().unwrap();
            
            idx = idx + 1;
            
            for i in idx..loop_size+tail_size {
                let node = Node::new(i);
                tail.next = Some(Box::new(node));
                tail = tail.next.as_mut().unwrap();
            }
            
            /*unsafe {
                tail.next = Some(Box::from_raw(loop_start as *mut Node));
            }*/
        }
        head
    }
    
    pub fn print_list(&self) {
        let mut current = Some(self);
        for _ in 0..20 {
            match current {
                Some(node) => {
                    print!("{} ", node.elem);
                    current = node.next.as_deref();
                }
                None => break,
            }
        }
        println!();
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.elem == other.elem
    }
}

impl Eq for Node {}



/*fn loop_size(node: Node) -> usize {
    todo!("Your code here!")
}*/

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
/*
#[cfg(test)]
mod sample_tests {

    fn assert_loop_size(tail_size: usize, loop_size: usize) {
        assert_eq!(
            super::loop_size(Node::gen_cycle(tail_size, loop_size)),
            loop_size
        );
    }

    #[test]
    fn four_nodes_with_a_loop_of_3() {
        assert_loop_size(1, 3);
    }

    #[test]
    fn no_tail_and_a_loop_of_4() {
        assert_loop_size(0, 4);
    }

    #[test]
    fn tiny_loop() {
        assert_loop_size(3, 1);
    }

    #[test]
    fn single_node() {
        assert_loop_size(0, 1);
    }
}*/