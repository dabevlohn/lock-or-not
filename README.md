## Mutex Code (main branch)

The mutex-based code demonstrates several classic concurrency design patterns adapted to Rust's ownership model:

### **Producer-Consumer Pattern**

- Multiple threads (`spawn_job`) act as **consumers** reading from shared `Arc<Mutex<Vec<T>>>`.
- `main()` acts as **producer** calling `add()` after spawning.
- Mutex serializes access to the shared vector, ensuring thread safety.


### **Shared Mutable State Pattern**

- `Arc<Mutex<Vec<T>>>` provides **shared ownership** (Arc) + **interior mutability** (Mutex).
- Classic RAII-style resource guarding: lock scope ensures data consistency.


### **Trait Object Polymorphism**

- `Arc<dyn Job + Send + Sync + 'static>` enables **dynamic dispatch** for different job types (`Ji`, `Js`).
- Follows **Strategy Pattern** — interchangeable behaviors (`run()`, `add()`) without changing client code.


### **Thread Pool / Worker Pattern**

- Vector of `JoinHandle`s collects spawned threads, waited on sequentially — basic **worker pool** precursor.


## Lock-Free Code (lock-free branch)

The SegQueue version showcases advanced **lock-free programming patterns**:

### **Lock-Free Producer-Consumer (MPMC Queue)**

- `SegQueue` implements **Michael-Scott (MS) Queue** algorithm using **CAS (Compare-And-Swap)** atomics.
- **Multi-Producer Multi-Consumer (MPMC)**: unlimited producers (`add()`) and consumers without blocking.
- **Helping Pattern**: threads assist each other's failed CAS operations to ensure progress (lock-freedom).


### **Atomic Reference Counting (Shared Ownership)**

- `Arc<SegQueue<T>>` maintains **lock-free reference counting** for the queue itself.
- No central lock — each `push()` uses segmented atomic operations.


### **Non-Blocking Delegation**

- Delegates **mutable operations** to the queue's internal lock-free linked list.
- **Wait-Free** elements (bounded number of steps per operation) via segmented design.


### **Trait Object + Lock-Free Integration**

- Same **Strategy/Polymorphism** as mutex version, but now with **lock-free data structure**.
- `Send + Sync + 'static` bounds ensure safe cross-thread delegation.


## Key Differences in Patterns

| Aspect | Mutex (Blocking) | SegQueue (Lock-Free) |
| :-- | :-- | :-- |
| **Synchronization** | Coarse-grained lock | Fine-grained atomics + CAS |
| **Progress Guarantee** | Mutual exclusion | Lock-freedom (no deadlocks) |
| **Scalability** | Poor (contention) | Excellent (no lock contention) |
| **Complexity** | Simple RAII | Advanced CAS loops + helping |

**Mutex**: Uses **Guarded Access Pattern** (simple, correct, scales poorly).
**SegQueue**: Uses **Lock-Free Queue Pattern** (complex, performant, scales linearly).

Both leverage **Rust's ownership + traits** for type-safe polymorphism, but SegQueue demonstrates sophisticated **concurrent data structure design** avoiding traditional locking entirely.
