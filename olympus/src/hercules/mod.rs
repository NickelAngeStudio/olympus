/*
 * @file hercules/mod.rs
 *
 * @module olympus::hercules
 *
 * @brief Contains header of Hercules, Worker, WorkerMessage and WorkOrder
 * 
 * @details
 * Contains header of struct Hercules, Worker, WorkerMessage and WorkOrder.
 * Implementation are in impl_hercules.rs, impl_worker.rs and impl_work_order.rs
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
 * Implement WorkOrder.
 * Document that closure are given when using push_labour
 */

use std::thread;
use std::sync::mpsc;

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

/// Worker that will execute the labour
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

enum WorkerMessage {
    NewLabour(Labour),
    Terminate,
}

/// Represent a labour that must be performed by Hercules.
type Labour = Box<dyn FnOnce() + Send + 'static>;

/// Implementations
mod impl_hercules;
mod impl_worker;

/// Tests
pub mod test_hercules;
