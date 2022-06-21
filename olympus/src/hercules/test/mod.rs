/*
 * @file hercules/test/mod.rs
 *
 * @module olympus::hercules
 *
 * @brief Contains header of API interface for hercules tests.
 * 
 * @details
 * Contains header of API interface for hercules tests.
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-06-21
 *
 * @version
 * 1.0 : 2022-06-21 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 */

 // Taskmaster Tests
#[cfg(test)]
mod taskmaster_test;

// Work order Tests
#[cfg(test)]
pub mod work_order_test;

// Worker Tests
#[cfg(test)]
mod worker_test;