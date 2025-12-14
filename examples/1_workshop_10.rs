use bumpalo::Bump;

pub struct BumpLinkedList<'a, T> {
    bump: &'a Bump,
    head: Option<&'a Node<'a, T>>,
}

struct Node<'a, T> {
    data: T,
    next: Option<&'a Node<'a, T>>,
}

impl<'a, T> BumpLinkedList<'a, T> {
    pub fn new(bump: &'a Bump) -> Self {
        BumpLinkedList { bump, head: None }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = self.bump.alloc(Node {
            data,
            next: self.head,
        });
        self.head = Some(new_node);
    }
}

fn main() {
    let bump = Bump::new();
    let mut list = BumpLinkedList::new(&bump);

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    // Память освобождается вся сразу, когда bump выходит из scope
    // Гораздо быстрее, чем individual deallocations!
}
