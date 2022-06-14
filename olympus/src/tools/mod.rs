/*
 * @file tools/mod.rs
 *
 * @module olympus::tools
 *
 * @brief Contains a collection of tools and dependencies abstractions.
 * 
 * @details
 * Contains a collection of tools and dependencies abstractions.
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-06-10
 *
 * @version
 * 1.0 : 2022-06-10 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 */

/// # Collection of tools and dependencies abstractions.
#[doc(inline)]
pub use core_count::get_logical_core_count;

/// Core counts 
#[doc(hidden)]
pub mod core_count;