pub struct Node {
    pub elem: usize,
    pub next: Option<Box<Node>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.elem == other.elem
    }
}

impl Eq for Node {}

impl Node {
    pub fn new(elem: usize) -> Self {
        Node { elem: elem, next: None}
    }
    
    pub fn gen_cycle(tail_size: usize) -> Node {
        let mut head = Node::new(0);
        
        let mut tail = &mut head;
        for i in 0..tail_size {
            let node = Node::new(i);
            tail.next = Some(Box::new(node));
            tail = tail.next.as_mut().unwrap();
        }
        
        head
    }
}

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