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
