use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::thread::{JoinHandle, spawn};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ThreadPool<F, T> {
    queue: Arc<Mutex<VecDeque<F>>>,
    workers: Vec<JoinHandle<()>>,
    results: Arc<Mutex<Vec<T>>>,
    joining: Arc<AtomicBool>
}

impl<F: Fn() -> T + Send + 'static, T: Send + 'static> ThreadPool<F, T> {
    pub fn new(size: u8) -> ThreadPool<F, T> {
        let mut pool = ThreadPool {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            workers: Vec::new(),
            results: Arc::new(Mutex::new(Vec::new())),
            joining: Arc::new(AtomicBool::new(false))
        };
        for _ in 0..size {
            let queue = pool.queue.clone();
            let results = pool.results.clone();
            let joining = pool.joining.clone();
            pool.workers.push(spawn(move || {
                loop {
                    let task: Option<F> = queue.lock().unwrap().pop_front();
                    if let Some(task) = task {
                        let result = task();
                        results.lock().unwrap().push(result);
                    }
                    else if joining.load(Ordering::Relaxed) {
                        break;
                    }
                }
            }));
        }
        return pool;
    }
    pub fn assign(&self, task: F) {
        self.queue.lock().unwrap().push_back(task);
    }
    pub fn join(self) -> Vec<T> {
        self.joining.store(true, Ordering::Relaxed);
        for worker in self.workers {
            worker.join();
        }
        if let Ok(results) = Arc::try_unwrap(self.results) {
            return results.into_inner().unwrap();
        }
        else {
            return Vec::new();
        }
    }
}