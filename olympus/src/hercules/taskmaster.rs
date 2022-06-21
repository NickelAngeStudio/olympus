/*
 * @file hercules/taskmaster.rs
 *
 * @module olympus::hercules
 *
 * @brief Contains definition of Taskmaster and the implementation.
 * 
 * @details
 * Contains definition of Taskmaster and the implementation.
 * The taskmaster is used to handle multi-threaded tasks and jobs.
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
use crate::tools;

/// ##### Struct used to distribute workload and labours to workers (aka Taskpool).
/// 
/// A taskmaster is a person who imposes a harsh or onerous workload on someone.
/// 
/// The taskmaster can have 1 to multiple workers to execute tasks.
/// # Examples
///
/// ``` 
/// // Get the taskmaster struct and new() options
/// use olympus::hercules::{Taskmaster, TaskmasterNewOptions}; 
/// 
/// // Create an instance of Taskmaster with maximum workers
/// let tsm = Taskmaster::new(TaskmasterNewOptions::MaximumWorkers);
/// 
/// // Push multiple labour to Taskmaster
/// for i in 0..100 {
///     tsm.push_labour(move || { println!("Hello world #{}", i); });
/// }
/// ```
pub struct Taskmaster {
    // Vector of workers
    workers: Vec<Worker>,

    // Sender of worker message
    sender: mpsc::Sender<WorkerMessage>,
}

/// Enum used as argument when using [`Taskmaster::new()`](struct.Taskmaster.html#method.new).
pub enum TaskmasterNewOptions {
    /// # Description
    /// Manually set taskmaster worker count.
    SetWorkerCount(usize),

    /// # Description
    /// Automatically use the hardware maximum [`CPU logical core`](https://www.cgdirector.com/cpu-cores-vs-logical-processors-threads/) count as worker count.
    MaximumWorkers,
}


impl Taskmaster {
    /// # Description
    /// Create a new instance of Taskmaster with 'TaskmasterNewOptions' worker(s).
    /// 
    /// # Argument(s)
    /// * `new_option` - [`TaskmasterNewOptions`](enum.TaskmasterNewOptions.html) enum used to set the size of taskmaster workers.
    /// # Return
    /// New instance of Taskmaster with 'TaskmasterNewOptions' worker(s).
    /// 
    /// # Panic(s)
    /// * 'new()' will panic if `TaskmasterNewOptions::SetWorkerCount` equal  0 or is higher than `TaskmasterNewOptions::MaximumWorkers`.
    pub fn new(new_option: TaskmasterNewOptions) -> Taskmaster {

        let mut _size = 0;

        // Define size according to TaskmasterNewOptions enum
        match new_option {
            TaskmasterNewOptions::SetWorkerCount(worker_count) => {
                _size = worker_count;
            },
            TaskmasterNewOptions::MaximumWorkers => {
                _size = tools::get_logical_core_count();
            },
        }

        // Size must be bigger thab 0
        assert!(_size > 0);

        // Size must be smaller or equals to logical core count
        assert!(_size <= tools::get_logical_core_count());

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(_size);

        for id in 0.._size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Taskmaster { workers, sender }
    }
    
    /// # Description
    /// Push a new labour to Taskmaster for execution.
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

/// Implementation of drop for Taskmaster
impl Drop for Taskmaster {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(WorkerMessage::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}