/*
 * @file hercules/work_order.rs
 *
 * @module olympus::hercules
 *
 * @brief Contains definition and implementation of work order.
 * 
 * @details
 * Contains definition and implementation of work order.
 * 
 * Work Orders are used to sync different labours together.
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

use std::sync::{Arc};
use std::sync::atomic::AtomicUsize;
use std::time::{Duration, Instant};
use crate::hercules::Taskmaster;

/// ##### Struct used to synchronize a batch of labours.
/// 
/// The Work order is used to synchronize a batch of labours.
/// To do so, use [`WorkOrder::add_labour()`](struct.WorkOrder.html#method.add_labour) to add 0..N labours
/// to the work order then use [`WorkOrder::wait()`](struct.WorkOrder.html#method.wait) to block the thread
/// until ALL jobs added in the work order are executed.
/// 
/// A wait [`Duration`](std::time::Duration) can be added to prevent infinite waiting.
/// 
/// Work order can be reused anytime after wait().
/// 
/// # Example(s)
///
/// ```
/// // Get the Taskmaster and WorkOrder structs and enum
/// use olympus::hercules::{Taskmaster, TaskmasterNewOptions, WorkOrder, WorkOrderWaitResult}; 
/// 
/// // For wait duration
/// use std::time::Duration;
/// 
/// // Create an instance of Taskmaster with logical core counts
/// let tsm = Taskmaster::new(TaskmasterNewOptions::MaximumWorkers);
/// 
/// // Create a Work Order for Taskmaster
/// let wo = WorkOrder::new(Some(&tsm));
/// 
/// // Add labours to the work order
/// wo.add_labour(move || { println!("Labour1"); });
/// wo.add_labour(move || { println!("Labour2"); });
/// wo.add_labour(move || { println!("Labour3"); });
/// 
/// // Wait 'Duration' for the order to be finished.
/// match wo.wait(Some(Duration::from_secs(5))) {
///     // All labours have been executed
///     WorkOrderWaitResult::Done => {
///         println!("All work order jobs finished!");
///     },
/// 
///     // Wait duration has expired
///     WorkOrderWaitResult::Timeout => {
///         panic!("Wait duration expired!");
///     }
/// }
/// ```
pub struct  WorkOrder<'a> {
    taskmaster: Option<&'a Taskmaster>,    // Unmutable reference to Taskmaster (or none)
    todo: Arc<AtomicUsize>,                // Labours left to do in the Work Order
}

/// Enum used as result returned from [`WorkOrder::wait()`](struct.WorkOrder.html#method.wait).
pub enum WorkOrderWaitResult {
    /// All added labours have been executed.
    Done, 
    
    /// Duration has expired.
    Timeout
}

impl<'a> WorkOrder<'a> {
    /// # Description
    /// Create a new instance of WorkOrder for referenced Taskmaster.
    /// 
    /// # Argument(s)
    /// * `taskmaster` - [`Taskmaster`](struct.Taskmaster.html) to push work order labours to.
    /// 
    /// # Return
    /// New instance of WorkOrder for referenced Taskmaster.
    /// 
    /// # Panic
    /// new() will panic if taskmaster == `None`
    pub fn new(taskmaster: Option<&'a Taskmaster>) -> WorkOrder {

        // Make sure that taskmaster argument is valid, else panic!
        if let None = taskmaster {
            panic!("WorkOrder::new() - Taskmaster argument can't be None");
        }

        // Return new WorkOrder
        WorkOrder { taskmaster, todo: Arc::new(AtomicUsize::new(0)) }
    }

    /// # Description
    /// Add labour to the work order, increasing the remaining labours todo by 1.
    /// 
    /// # Argument(s)
    /// * `f` - Closure that will be executed once a worker is available.
    pub fn add_labour<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let todo = self.todo.clone();

        // Increment todos
        todo.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.taskmaster.as_ref().unwrap().push_labour(Box::new(move || {
            // Execute labour
            f();

            // Decrement todo
            todo.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        }));
    }

    /// # Descriptions
    /// Block the thread until all work order labours are executed or `timeout` is reached.
    /// 
    /// # Argument(s)
    /// * `timeout` - Max waiting time. Will return `WorkOrderWaitResult::Timeout` status if reached.
    /// 
    /// # Warning(s)
    /// If `timeout` is set to None, this function might run indefinitely.
    /// 
    /// # Return 
    /// * [`WorkOrderWaitResult`](enum.WorkOrderWaitResult.html) result of waiting.
    pub fn wait(&self, timeout: Option<Duration>) -> WorkOrderWaitResult {

        match timeout {
            Some(timeout) => {

                // Time we started to wait
                let started = Instant::now();

                // Waiting loop
                while self.todo.load(std::sync::atomic::Ordering::Relaxed) > 0 && Instant::now() - started <= timeout { }

                // Verify if waiting has timed out
                if Instant::now() - started > timeout {
                    // Return Timeout as result
                    return WorkOrderWaitResult::Timeout;
                }
            },
            None => {
                // Waiting loop (Can run indefinitely since no timeout specified.)  
                while self.todo.load(std::sync::atomic::Ordering::Relaxed) > 0 {}        
            },
        }

        // All jobs are done
        WorkOrderWaitResult::Done
    }

    /// # Descriptions
    /// Get the count of remaining labour to do.
    /// 
    /// # Return 
    /// Count of remaining labour to do.
    pub fn get_labour_remaining(&self) -> usize {
        self.todo.load(std::sync::atomic::Ordering::Relaxed)
    }

}