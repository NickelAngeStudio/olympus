
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