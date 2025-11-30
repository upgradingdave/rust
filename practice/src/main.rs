use practice::swap_node_pairs::Node;

fn main() {
    let mut list = Node{
        next: Some(Box::new(Node{
            next: Some(Box::new(Node{
                next: None,
            }))
        }))
    };
    
    println!("{}", Node::display_option(&list.next));
}
