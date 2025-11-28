use std::sync::{Arc, Mutex};
use std::thread;

trait Job {
    fn run(&self, n: usize);
    fn add(&self);
}

struct Ji {
    a: Arc<Mutex<Vec<i8>>>, // Mutex для изменяемого доступа
}

struct Js {
    a: Arc<Mutex<Vec<String>>>,
}

impl Job for Ji {
    fn run(&self, n: usize) {
        if n < self.a.lock().unwrap().len() {
            println!(
                "Job {:?}: a[{}] = {:?}, lc: {}",
                std::thread::current().id(),
                n,
                self.a.lock().unwrap()[n],
                Arc::strong_count(&self.a)
            );
        }
    }

    fn add(&self) {
        self.a.lock().unwrap().push(8); // Теперь работает!
    }
}

impl Job for Js {
    fn run(&self, n: usize) {
        if n < self.a.lock().unwrap().len() {
            println!(
                "Mob {:?}: a[{}] = {:?}, lc: {}",
                std::thread::current().id(),
                n,
                &self.a.lock().unwrap()[n],
                Arc::strong_count(&self.a)
            );
        }
    }

    fn add(&self) {
        self.a.lock().unwrap().push("B".to_string());
    }
}

fn spawn_job(job: Arc<dyn Job + Send + Sync + 'static>, n: usize) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        job.run(n);
    })
}

fn spawn_add(job: Arc<dyn Job + Send + Sync + 'static>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        job.add();
    })
}

fn main() {
    let di = Arc::new(Mutex::new(vec![1; 10]));
    let ds = Arc::new(Mutex::new(vec!["A".to_string(); 10]));

    let ji = Ji { a: Arc::clone(&di) };
    let js = Js { a: Arc::clone(&ds) };

    let jobi = Arc::new(ji) as Arc<dyn Job + Send + Sync + 'static>;
    let jobs = Arc::new(js) as Arc<dyn Job + Send + Sync + 'static>;

    let mut handles = vec![];

    // Запуск run() для всех
    for n in 0..13 {
        let hir = spawn_job(Arc::clone(&jobi), n);
        let hia = spawn_add(Arc::clone(&jobi));
        let hsr = spawn_job(Arc::clone(&jobs), n);
        let hsa = spawn_add(Arc::clone(&jobs));
        handles.push(hir);
        handles.push(hia);
        handles.push(hsr);
        handles.push(hsa);
    }

    // Ждём завершения run()
    for handle in handles {
        handle.join().unwrap();
    }
}
