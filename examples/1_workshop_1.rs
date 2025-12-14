struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data, next: None }
    }

    fn push(&mut self, data: T) {
        match self.next {
            None => self.next = Some(Box::new(Node::new(data))),
            Some(ref mut node) => node.push(data),
        }
    }
}

fn main() {
    let mut head = Node::new(1);
    head.push(2);
    head.push(3);
    println!(
        "Список: {} -> {} -> {}",
        head.data,
        head.next.as_ref().unwrap().data,
        head.next.as_ref().unwrap().next.as_ref().unwrap().data
    );
}
