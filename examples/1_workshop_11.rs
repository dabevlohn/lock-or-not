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
