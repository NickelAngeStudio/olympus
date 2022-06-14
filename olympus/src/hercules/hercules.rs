/*
 * @file hercules/hercules.rs
 *
 * @module olympus::hercules::hercules
 *
 * @brief Contains definition of Hercules and the implementation
 * 
 * @details
 * Contains definition of Hercules and the implementation.
 * Hercules is an expert of labour. He handles multi-threaded tasks and jobs.
 * 
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-06-14
 *
 * @version
 * 1.0 : 2022-06-14 | Mathieu Grenier | Code creation
 *
 * @ref
 * The Rust Programming Language guide : https://doc.rust-lang.org/book/ch20-02-multithreaded.html
 * 
 * @todo

 */

use std::sync::{mpsc, Arc, Mutex};
use super::worker::{Worker, WorkerMessage};

/// Hercules is an expert of labour. He handles multi-threaded tasks and jobs.
/// 
/// # Examples
///
/// ```
/// // Crate that get the count of logical cores this process could try to use
/// extern crate num_cpus;  
/// 
/// // Get the Hercules struct
/// use olympus::Hercules; 
/// 
/// // Create an instance of Hercules with logical core counts
/// let hercules = Hercules::new(num_cpus::get());
/// 
/// // Push labour to Hercules
/// hercules.push_labour(|| { println!("Hello world"); });
/// ```
pub struct Hercules {
    workers: Vec<Worker>,
    sender: mpsc::Sender<WorkerMessage>,
}



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