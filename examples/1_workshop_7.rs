use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: RefCell<Option<Rc<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<Self> {
        Rc::new(Node {
            data,
            next: RefCell::new(None),
        })
    }

    fn push(&self, data: T) {
        let new_node = Node::new(data);
        *self.next.borrow_mut() = Some(Rc::clone(&new_node));
    }
}

fn main() {
    let head = Node::new(1);
    head.push(2);
    head.push(3);

    println!("{:?}", head);
    // Node { data: 1, next: RefCell { value: Some(...) } }
}
