//http://codewars.com/kata/59c6f43c2963ecf6bf002252

use std::fmt;

pub struct Node {
    pub next: Option<Box<Node>>
}
impl Node {
    /// Display method for &Option<Box<Node>>; useful for debugging a linked list.
    /// Format: [0xabbc --> 0xbcdd --> 0xabcd]
    pub fn display_option<'a>(node: &'a Option<Box<Node>>) -> impl fmt::Display + 'a {
        match node {
            None => "[]".to_string(),
            Some(node) => format!("{:?} --> {}", node as *const _, Node::display_option(&node.next)),
        }
    }
}

/// Display format: 0xabbc --> 0xbcdd --> 0xabcd
impl fmt::Display for Node { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} --> {}", self as *const _, Node::display_option(&self.next))
    }
}