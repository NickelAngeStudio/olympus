/*
 * @file hercules/worker.rs
 *
 * @module olympus::hercules::worker
 *
 * @brief Contains definition and implementation of Worker used to execute labour.
 * 
 * @details
 * Contains header of Worker used to execute labour. Hercules need 1..N worker
 * to execute his labours.
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

use std::{sync::{Arc, Mutex, mpsc}, thread};

/// Worker that will execute the labour
pub struct Worker {
    pub id: usize,                                  // Id of the worker (not unique)
    pub thread: Option<thread::JoinHandle<()>>,     // Running thread of the worker
}

/// Message sent to worker.
pub enum WorkerMessage {
    NewLabour(Labour),                          // Send a new labour to the worker
    Terminate,                                  // Tell the worker to terminate and join the thread
}

/// Represent a labour that must be performed by Hercules.
type Labour = Box<dyn FnOnce() + Send + 'static>;

/// Implementation of worker
impl Worker {
    /// # Description
    /// Create a new instance of Worker with an id and a receiver.
    /// 
    /// # Argument(s)
    /// * `id` - Id to identify the worker. Doesn't need to be unique.
    /// * `receiver` - Receiver used to receive message.
    /// 
    /// # Return
    /// New instance of Worker with a running thread.
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<WorkerMessage>>>) -> Worker {

        let thread = thread::spawn(move || loop {
            // Worker is waiting for a job
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                WorkerMessage::NewLabour(labour) => {
                    labour();
                }
                WorkerMessage::Terminate => {
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}