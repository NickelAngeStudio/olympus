/*
 * @file hercules/impl_hercules.rs
 *
 * @module olympus::hercules
 *
 * @brief Contains implementation of struct Hercules.
 * 
 * @details
 * Contains implementation of struct Hercules such as new, push_labour and drop.
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-06-08
 *
 * @version
 * 1.0 : 2022-06-08 | Mathieu Grenier | Code creation
 *
 * @ref
 * The Rust Programming Language guide : https://doc.rust-lang.org/book/ch20-02-multithreaded.html
 * 
 * @todo
 */
use super::Hercules;
use super::Worker;
use super::WorkerMessage;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;


impl Hercules {
    /// # Description
    /// Create a new instance of Hercules with 'n = size' worker(s).
    /// 
    /// # Argument(s)
    /// * `size` - Number of available workers for labours
    /// 
    /// # Return
    /// New instance of Hercules with 'n = size' worker(s).
    /// 
    /// # Panic
    /// new() will panic if size == 0
    pub fn new(size: usize) -> Hercules {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Hercules { workers, sender }
    }
    
    /// # Description
    /// Push a new labour to Hercules for execution.
    /// 
    /// # Argument(s)
    /// * `f` - Closure that will be executed once a worker is available.
    pub fn push_labour<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let labour = Box::new(f);

        self.sender.send(WorkerMessage::NewLabour(labour)).unwrap();
    }
}

/// Implementation of drop for Hercules
impl Drop for Hercules {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(WorkerMessage::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}