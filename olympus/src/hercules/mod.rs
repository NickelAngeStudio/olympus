/*
 * @file hercules/mod.rs
 *
 * @module olympus::hercules
 *
 * @brief Contains header of API interface for hercules components.
 * 
 * @details
 * Contains header of API interface for hercules components.
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
 * 
 * @todo
 */


 /// # Re-export of Taskmaster for Public API
#[doc(inline)]
pub use taskmaster::Taskmaster as Taskmaster;
pub use taskmaster::TaskmasterNewOptions as TaskmasterNewOptions;

 /// # Re-export of Work Order for Public API
 #[doc(inline)]
//pub use work_order::{WorkOrder, WorkOrderWaitResult} as hercules;
pub use work_order::WorkOrder as WorkOrder;
pub use work_order::WorkOrderWaitResult as WorkOrderWaitResult;

// Taskmaster definition and implementation
#[doc(hidden)]
pub mod taskmaster;

// Worker definition and implementation
#[doc(hidden)]
pub(in crate::hercules) mod worker;

// Work Order definition and implementation
#[doc(hidden)]
pub mod work_order;

// hercules module tests
#[cfg(test)]
mod test;