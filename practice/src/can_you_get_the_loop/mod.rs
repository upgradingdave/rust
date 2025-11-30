pub struct Node {
    pub elem: usize,
    pub next: Option<Box<Node>>,
}

impl Node {
    fn new(elem: usize) -> Self {
        Node { elem: elem, next: None }
    }
    

    fn create_list(size: usize, start_idx: usize) -> Node {
        let mut idx = start_idx;
        let mut head = Self::new(idx);
        idx = idx + 1;
        let mut tail = &mut head;
        
        for i in idx..size+start_idx {
            let node = Self::new(i);
            tail.next = Some(Box::new(node));
            tail = tail.next.as_mut().unwrap();
        }
        
        head
    }
    
    pub fn gen_cycle(tail_size: usize, loop_size: usize) -> Self {
        
        let list1 = Self::create_list(tail_size, 0);
        
        if loop_size > 0 {
            let _list2 = Self::create_list(loop_size, tail_size);
            //list1.next = Some(Box::new(list2));
        }
        list1
    }
    
    pub fn print_list(&self) {
        let mut current = Some(self);
        loop {
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