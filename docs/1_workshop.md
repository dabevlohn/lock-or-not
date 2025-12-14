## Rust Workshop: Связные структуры данных и Borrow Checker

---

### 1. Как написать связные структуры данных на safe Rust?

**Основная сложность:** Rust требует явного управления владением. В связных структурах каждый узел владеет следующим, но нужна гибкость для разных паттернов доступа.

**Наивный подход (РАБОТАЕТ для однонаправленных списков):**

```rust
// ✅ Простая однонаправленная связная структура
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
    println!("Список: {} -> {} -> {}", head.data,
             head.next.as_ref().unwrap().data,
             head.next.as_ref().unwrap().next.as_ref().unwrap().data);
}
```

**Почему это работает:** `Box<T>` — уникальное владение, чёткая иерархия, нет циклических ссылок.

---

### 2. Реализация структур данных: LinkedList, BTreeMap, Graph

#### LinkedList на safe Rust

```rust
use std::fmt;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut current = &self.head;
        while let Some(node) = current {
            write!(f, "{} -> ", node.data)?;
            current = &node.next;
        }
        write!(f, "None")
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    println!("{}", list);  // 3 -> 2 -> 1 -> None

    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.peek(), Some(&2));
}
```

#### BTreeMap (используем стандартную библиотеку)

```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert("Alice", 25);
    map.insert("Bob", 30);
    map.insert("Charlie", 35);

    // Итерация в отсортированном порядке
    for (name, age) in map.iter() {
        println!("{}: {}", name, age);
    }
    // Вывод: Alice: 25, Bob: 30, Charlie: 35

    // Поиск
    if let Some(age) = map.get("Bob") {
        println!("Bob's age: {}", age);
    }

    // Range queries
    for (name, age) in map.range("Alice"..="Bob") {
        println!("{}: {}", name, age);
    }
}
```

#### Graph на safe Rust (с индексами)

```rust
pub struct Graph<T> {
    nodes: Vec<Node<T>>,
}

struct Node<T> {
    data: T,
    edges: Vec<usize>,  // Индексы соседних узлов
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, data: T) -> usize {
        let index = self.nodes.len();
        self.nodes.push(Node {
            data,
            edges: Vec::new(),
        });
        index
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.nodes.len() && to < self.nodes.len() {
            self.nodes[from].edges.push(to);
        }
    }

    pub fn neighbors(&self, node: usize) -> Option<&[usize]> {
        self.nodes.get(node).map(|n| n.edges.as_slice())
    }

    pub fn get_data(&self, node: usize) -> Option<&T> {
        self.nodes.get(node).map(|n| &n.data)
    }
}

fn main() {
    let mut graph = Graph::new();

    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    graph.add_edge(a, b);
    graph.add_edge(b, c);
    graph.add_edge(a, c);

    println!("Neighbors of A: {:?}", graph.neighbors(a)); // [1, 2]

    // DFS
    fn dfs<T: std::fmt::Debug>(
        graph: &Graph<T>,
        node: usize,
        visited: &mut Vec<bool>,
    ) {
        visited[node] = true;
        println!("Visiting: {:?}", graph.get_data(node));

        if let Some(neighbors) = graph.neighbors(node) {
            for &neighbor in neighbors {
                if !visited[neighbor] {
                    dfs(graph, neighbor, visited);
                }
            }
        }
    }

    let mut visited = vec![false; 3];
    dfs(&graph, a, &mut visited);
}
```

---

### 3. Почему borrow checker бракует наивную реализацию?

#### Проблема 1: Циклические ссылки

```rust
// ❌ ЭТО НЕ КОМПИЛИРУЕТСЯ
struct Node<T> {
    data: T,
    next: Option<&mut Node<T>>,    // ❌ Problematic!
    prev: Option<&mut Node<T>>,    // ❌ Problematic!
}
```

**Почему:** Borrow checker не может гарантировать, что ссылки будут валидны:

- Если у узла есть `next`, а у того есть `prev`, получается циклическое заимствование
- Невозможно безопасно удалить узел, так как на него ссылаются другие

#### Проблема 2: Множественное заимствование

```rust
// ❌ ЭТО НЕ КОМПИЛИРУЕТСЯ
struct Node<T> {
    data: T,
    next: Option<&Node<T>>,
}

fn main() {
    let node1 = Node { data: 1, next: None };
    let node2 = Node { data: 2, next: Some(&node1) };
    let node3 = Node { data: 3, next: Some(&node1) };

    // ❌ node1 заимствована дважды! Borrow checker это запретит.
}
```

#### Проблема 3: Lifetime issues

```rust
// ❌ ЭТО НЕ КОМПИЛИРУЕТСЯ
struct LinkedList<'a, T> {
    head: Option<&'a Node<T>>,
}

impl<'a, T> LinkedList<'a, T> {
    fn push_front(&mut self, data: T) {
        // ❌ Как создать новый Node, если у нас есть &'a ссылка?
        // Lifetimes не совпадают!
    }
}
```

---

### 4. Как обойти требования borrow checker с помощью Rc и RefCell?

#### Rc (Reference Counting) — множественное владение (read-only)

```rust
use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<Self> {
        Rc::new(Node {
            data,
            next: None,
        })
    }
}

fn main() {
    let node1 = Node::new(1);
    let node2 = Node::new(2);
    let node3 = Rc::new(Node {
        data: 3,
        next: Some(Rc::clone(&node1)),  // ✅ node1 используется дважды!
    });

    println!("node1 refs: {}", Rc::strong_count(&node1)); // 2
}
```

#### RefCell (Interior Mutability) — mutable доступ

```rust
use std::rc::Rc;
use std::cell::RefCell;

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
```

#### Rc + RefCell для двусвязного списка

```rust
use std::rc::Rc;
use std::cell::RefCell;

type NodeRef<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<NodeRef<T>>,
    prev: Option<NodeRef<T>>,  // ✅ Теперь это работает!
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
}
```

---

### 5. Чем полезен Weak?

**Проблема:** Rc + циклические ссылки = утечка памяти!

```rust
// ❌ УТЕЧКА ПАМЯТИ
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,  // ❌ Циклическая ссылка!
}

// Если A.next -> B, B.prev -> A, то:
// A имеет strong count = 2 (self + B.prev)
// B имеет strong count = 2 (self + A.next)
// Когда мы удаляем список, оба остаются в памяти!
```

**Решение: Weak references**

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

type NodeRef<T> = Rc<RefCell<Node<T>>>;
type WeakNodeRef<T> = Weak<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<NodeRef<T>>,
    prev: Option<WeakNodeRef<T>>,  // ✅ Weak вместо Rc!
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
            new_node.borrow_mut().prev = Some(Rc::downgrade(tail));  // ✅ Weak!
        } else {
            self.head = Some(Rc::clone(&new_node));
        }

        self.tail = Some(new_node);
    }

    fn traverse(&self) {
        if let Some(head) = &self.head {
            let mut current = Some(Rc::clone(head));
            while let Some(node) = current {
                println!("{:?}", node.borrow().data);
                current = node.borrow_mut().next.take().or_else(|| {
                    node.borrow()
                        .prev
                        .as_ref()
                        .and_then(|prev| prev.upgrade())  // ✅ upgrade() преобразует Weak -> Rc
                });
            }
        }
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    list.traverse();
    // Когда list выходит из scope, память корректно освобождается!
}
```

**Когда использовать Weak:**

- 🔗 Циклические графы (parent <-> child relationships)
- 🎛️ Event listeners и observer patterns
- 🌳 Tree structures с parent pointers

---

### 6. Как unsafe-код может помочь оптимизировать реализацию?

#### Пример 1: Быстрое удаление элементов из связного списка

```rust
use std::ptr::NonNull;

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
        LinkedList {
            head: None,
            len: 0,
        }
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
            self.head.map(|head| {
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
}
```

#### Пример 2: Custom allocator для ещё большей производительности

```rust
// Используем bumpalo для fast allocation
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
        BumpLinkedList {
            bump,
            head: None,
        }
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
```

#### Пример 3: Intrusive list (максимальная оптимизация)

```rust
// Узел содержит ссылку на себя, а не на данные!
// Это позволяет избежать дополнительных allocations

struct ListNode {
    next: Option<*mut ListNode>,
    prev: Option<*mut ListNode>,
}

impl ListNode {
    fn container_of<T>(ptr: *mut ListNode) -> *mut T
    where
        T: HasNode,
    {
        unsafe {
            let offset = std::mem::offset_of!(T, node);
            (ptr as *mut u8).sub(offset) as *mut T
        }
    }
}

trait HasNode {
    fn node(&mut self) -> &mut ListNode;
}

struct MyData {
    value: i32,
    node: ListNode,
}

impl HasNode for MyData {
    fn node(&mut self) -> &mut ListNode {
        &mut self.node
    }
}

// Это максимально оптимизированный подход:
// - Нет дополнительных allocations
// - Cache-friendly
// - О(1) для всех операций
```

---

## Итоговая таблица решений

| Задача                              | Инструмент                | Pros              | Cons                   |
| :---------------------------------- | :------------------------ | :---------------- | :--------------------- |
| **Однонаправленный список**         | `Box<Option<Box<T>>>`     | Безопасно, просто | Медленно               |
| **Двусвязный список**               | `Rc<RefCell<T>>` + `Weak` | Безопасно, гибко  | Overhead от Rc/RefCell |
| **Граф с циклами**                  | Индексированный граф      | Быстро, безопасно | Сложнее логика         |
| **Максимальная производительность** | `unsafe` + raw pointers   | Скоростно         | Небезопасно, сложно    |
| **Cache-friendly структуры**        | Intrusive lists           | Супер быстро      | Требует unsafe         |

---
