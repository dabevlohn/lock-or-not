use crossbeam_queue::SegQueue;
use std::sync::Arc;
use std::thread;
// use std::thread::{self, sleep};
// use std::time::Duration;

trait Job {
    fn run(&self);
    fn add(&self, value: i8);
}

struct Ji {
    a: Arc<SegQueue<i8>>, // Используем неблокирующую очередь
}

impl Job for Ji {
    fn run(&self) {
        // Нельзя индексировать, просто демонстрируем размер
        println!(
            "Job {:?}: количество элементов примерно {}",
            std::thread::current().id(),
            self.a.len()
        );
    }

    fn add(&self, value: i8) {
        self.a.push(value); // Lock-free добавление
    }
}

fn spawn_job(job: Arc<dyn Job + Send + Sync + 'static>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        job.run();
        // sleep(Duration::from_millis(2));
    })
}

fn spawn_add(job: Arc<dyn Job + Send + Sync + 'static>, i: i8) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        job.add(i);
        // sleep(Duration::from_millis(2));
    })
}

fn main() {
    let queue = Arc::new(SegQueue::new());
    let ji = Ji {
        a: Arc::clone(&queue),
    };
    let job = Arc::new(ji) as Arc<dyn Job + Send + Sync + 'static>;

    // Запустим несколько потоков, которые будут читать состояние
    let mut handles = vec![];
    for i in 0..10 {
        let hr = spawn_job(Arc::clone(&job));
        let ha = spawn_add(Arc::clone(&job), i);
        handles.push(hr);
        handles.push(ha);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Всего элементов после добавления: {}", queue.len());
}
