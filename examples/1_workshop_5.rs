// ❌ ЭТО НЕ КОМПИЛИРУЕТСЯ
struct Node1<'a, T> {
    data: T,
    next: Option<&'a mut Node1<'a, T>>, // ❌ Problematic!
                                        //    prev: Option<&'a mut Node1<'a, T>>, // ❌ Problematic!
}

// ❌ ЭТО НЕ КОМПИЛИРУЕТСЯ
struct Node2<'a, T> {
    data: T,
    next: Option<&'a Node2<'a, T>>,
}

fn main() {
    let mut node1 = Node1 {
        data: 1,
        next: None,
        //        prev: None,
    };
    let mut node2 = Node1 {
        data: 2,
        next: None,
        //        prev: Some(&mut node1),
    };
    let mut node3 = Node1 {
        data: 3,
        next: Some(&mut node1),
        //        prev: Some(&mut node2),
    };

    // ❌ node1 заимствована дважды! Borrow checker это запретит.
}

// ❌ ЭТО НЕ КОМПИЛИРУЕТСЯ
// struct LinkedList<'a, T> {
//     head: Option<&'a Node1<T>>,
// }
//
// impl<'a, T> LinkedList<'a, T> {
//     fn push_front(&mut self, data: T) {
//         // ❌ Как создать новый Node, если у нас есть &'a ссылка?
//         // Lifetimes не совпадают!
//     }
// }
