/*
 * @file hercules/work_order.rs
 *
 * @module olympus::hercules::work_order
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

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::Hercules;

/// A Work Order is used to synchronize multiple labours with Hercules.
/// 
/// Some labour needs to be done before another task and the work order
/// is the tool to achieve that. 
/// Hercules can handle 1 to many work orders.
/// 
/// # Example(s)
///
/// ```
/// // Get olympus tools for core count
/// use olympus::tools; 
/// 
/// // Get the Hercules and WorkOrder structs
/// use olympus::Hercules; 
/// use olympus::WorkOrder;
/// 
/// // For wait duration
/// use std::time::Duration;
/// 
/// // Create an instance of Hercules with logical core counts
/// let hercules = Hercules::new(tools::get_logical_core_count());
/// 
/// // Create a Work Order for Hercules
/// let wo = WorkOrder::new(Some(&hercules));
/// 
/// // Add labours to the work order
/// wo.add_labour(move || { println!("Labour1"); });
/// wo.add_labour(move || { println!("Labour2"); });
/// wo.add_labour(move || { println!("Labour3"); });
/// 
/// // Wait 'Duration' for the order to be finished.
/// wo.wait(Some(Duration::from_secs(5)));
/// ```
pub struct  WorkOrder<'a> {
    hercules: Option<&'a Hercules>,             // Unmutable reference to Hercules (or none)
    todo: WorkOrderTodo,                        // Labours left to do in the Work Order
}

/// Wait result returned with work_order.wait()
pub enum WorkOrderWaitResult {
    Done,                     // All labours have been done
    Timeout                   // Waiting has timedout
}


// Work Order labours remaining type
type WorkOrderTodo = Arc<Mutex<Box<usize>>>;


impl<'a> WorkOrder<'a> {
    /// # Description
    /// Create a new instance of WorkOrder for referenced Hercules.
    /// 
    /// # Argument(s)
    /// * `hercules` - Hercules to push work order labours to.
    /// 
    /// # Return
    /// New instance of WorkOrder for referenced Hercules.
    /// 
    /// # Panic
    /// new() will panic if hercules == `None`
    pub fn new(hercules: Option<&'a Hercules>) -> WorkOrder {

        // Make sure that hercules argument is valid, else panic!
        if let None = hercules {
            panic!("WorkOrder::new() - hercules argument can't be None");
        }

        // Shared count of labours to do.
        let todo = Arc::new(Mutex::new(Box::new(0 as usize)));

        // Return new WorkOrder
        WorkOrder { hercules, todo }
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

        // Increment todo
        *todo.lock().unwrap().as_mut() += 1;

        self.hercules.as_ref().unwrap().push_labour(Box::new(move || {
            // Execute labour
            f();

            // Decrement todo
            *todo.lock().unwrap().as_mut() -= 1;
        }));
    }

    /// # Descriptions
    /// Block the thread until all work order labours are executed or `timeout` is reached.
    /// 
    /// # Argument(s)
    /// * `timeout` - Max waiting time. Will return `WorkOrderWaitResult::Timeout` status if reached.
    /// 
    /// # Note(s)
    /// If `timeout` is set to None, this function might run indefinitely.
    /// 
    /// # Return 
    /// * `WorkOrderWaitResult::Done` - All labours of the work order have been executed.
    /// * `WorkOrderWaitResult::Timeout` - Waiting has timed out (only if timeout != None).
    pub fn wait(&self, timeout: Option<Duration>) -> WorkOrderWaitResult {

        match timeout {
            Some(timeout) => {

                // Time we started to wait
                let started = Instant::now();

                // Waiting loop
                while *self.todo.lock().unwrap().as_ref() > 0 && Instant::now() - started <= timeout { }

                // Verify if waiting has timed out
                if Instant::now() - started > timeout {
                    // Return Timeout as result
                    return WorkOrderWaitResult::Timeout;
                }
            },
            None => {
                // Waiting loop (Can run indefinitely since no timeout specified.)
                while *self.todo.lock().unwrap().as_ref() > 0 {}        
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
        *self.todo.lock().unwrap().as_ref()
    }

}