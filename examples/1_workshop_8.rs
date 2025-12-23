use std::cell::RefCell;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<NodeRef<T>>,
    prev: Option<NodeRef<T>>, // ✅ Теперь это работает!
}

impl<T> Node<T> {
    fn new(data: T) -> NodeRef<T> {
        Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }))
    }
}

#[derive(Debug)]
struct DoublyLinkedList<T> {
    head: Option<NodeRef<T>>,
    tail: Option<NodeRef<T>>,
}

impl<T> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, data: T) {
        let new_node = Node::new(data);

        if let Some(tail) = &self.tail {
            tail.borrow_mut().next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(Rc::clone(tail));
        } else {
            self.head = Some(Rc::clone(&new_node));
        }

        self.tail = Some(new_node);
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    println!("{:?}", list);
}
