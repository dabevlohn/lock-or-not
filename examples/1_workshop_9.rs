use std::ptr::NonNull;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    len: usize,
}

struct Node<T> {
    data: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Box::leak(Box::new(Node {
            data,
            next: self.head,
        }));

        // ✅ NonNull::new_unchecked безопаснее, так как Box::leak гарантирует non-null
        self.head = Some(unsafe { NonNull::new_unchecked(new_node) });
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.head.map(|mut head| {
                let head_ref = head.as_mut();
                let next = head_ref.next;
                let data = std::ptr::read(&head_ref.data);

                // ✅ Освобождаем память узла
                let _ = Box::from_raw(head.as_ptr());

                self.head = next;
                self.len -= 1;
                data
            })
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.len, 2);

    println!("{:?}", list);
}
