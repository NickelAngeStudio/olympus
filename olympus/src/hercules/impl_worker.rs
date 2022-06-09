use super::Worker;
use super::WorkerMessage;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;


/// Implementation of worker
impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<WorkerMessage>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                WorkerMessage::NewLabour(labour) => {
                    println!("Worker {} got a labour; executing.", id);

                    labour();
                }
                WorkerMessage::Terminate => {
                    println!("Worker {} was told to terminate.", id);

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