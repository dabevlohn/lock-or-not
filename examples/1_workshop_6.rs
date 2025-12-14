use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<Self> {
        Rc::new(Node { data, next: None })
    }
}

fn main() {
    let node1 = Node::new(1);
    let node2 = Node::new(2);
    let node3 = Rc::new(Node {
        data: 3,
        next: Some(Rc::clone(&node1)), // ✅ node1 используется дважды!
    });

    println!("node1 refs: {}", Rc::strong_count(&node1)); // 2
}
