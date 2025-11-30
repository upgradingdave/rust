use std::cell::RefCell;

thread_local! {
    /// Per-thread linked-list simulation; `ARENA[n]` holds the index for the
    /// next node that node `n` links to. Typically `ARENA[n] = n + 1` but the
    /// last entry should be a value in the range `0..=n - 1`.
    static ARENA: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Node(usize);

impl Node {
    /// Generate a linked list containing `tail_size + loop_size` nodes, with a loop
    /// `loop_size` long.
    pub fn gen_cycle(tail_size: usize, loop_size: usize) -> Node {
        debug_assert!(loop_size > 0);
        // NOTE: This _replaces_ the previous arena contents. This is fine here
        // because we only need one linked-list of nodes to exist per test, and
        // tests run in their own threads.
        ARENA.with(move |arena| {
            let mut vec: Vec<usize> = (1..=tail_size + loop_size).collect();
            *vec.last_mut().unwrap() = tail_size;
            *arena.borrow_mut() = vec;
        });
        Node(0)
    }

    pub fn next(&self) -> Node {
        Node(ARENA.with(|a| a.borrow()[self.0]))
    }
}