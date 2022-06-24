/*
 * @file tools/core_count.rs
 *
 * @module olympus::tools
 *
 * @brief Contains abstraction function used to get the logical core count.
 * 
 * @details
 * Contains abstraction function used to get the logical core count.
 * 
 * Depedencies should be abstracted as much as possible bacause a new solution can arise
 * or they can stop being maintained.
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


/// Dependency crate used to get logical core count
extern crate num_cpus;

/// Get the count of logical cores
/// 
/// # Example
/// ```
/// // Use Olympus tools
/// use olympus::tools; 
/// 
/// // Get the logical core count
/// println!("Count of logical core = {}", tools::get_logical_core_count());
/// ```
/// # Return
/// Count of logical cores or 1 if not available
/// 
/// # Note(s)
/// Uses crate [`num_cpus`]
/// 
/// # Reference(s)
/// [https://www.cgdirector.com/cpu-cores-vs-logical-processors-threads/](https://www.cgdirector.com/cpu-cores-vs-logical-processors-threads/)
pub fn get_logical_core_count() -> usize {
    num_cpus::get()
}

#[cfg(test)]
mod test {
    /// Test that get_logical_core_count() return > 0
    #[test]
    fn get_logical_core_count() {
        use crate::tools; 
        println!("Count of logical core = {}", tools::get_logical_core_count());
        assert!(tools::get_logical_core_count() > 0)
    }
}