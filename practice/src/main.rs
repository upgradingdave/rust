pub mod can_you_get_the_loop;

use can_you_get_the_loop::Node;

fn main() {
    let list = Node::gen_cycle(3, 3);
    Node::print_list(&list);
}
