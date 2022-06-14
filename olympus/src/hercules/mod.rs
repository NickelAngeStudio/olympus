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


// Hercules definition and implementation
pub mod hercules;

// Worker definition and implementation
pub(in crate::hercules) mod worker;

// Work Order definition and implementation
pub mod work_order;



/// Implementations
//mod impl_hercules;
//mod impl_worker;
//mod impl_work_order;

/// Tests
mod test_hercules;

#[cfg(test)]
mod test_work_order;
