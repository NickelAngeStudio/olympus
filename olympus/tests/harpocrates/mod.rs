/*
 * @file tests/harpocrates/mod.rs
 *
 * @module olympus::tests::harpocrates
 *
 * @brief Contains reusable functions and tests for harpocrates traits integration.
 * 
 * @details
 * Contains reusable functions and tests for harpocrates traits integration.
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-06-27
 *
 * @version
 * 1.0 : 2022-06-27 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 */


// Contains the reusable functions used to test Traits
pub mod function;

// Contains traits integrations
pub mod integration;

#[cfg(test)]
mod tests {
    use super::{function::secret_tests, integration::basic_secret::BasicSecret};


    #[test]
    fn basic_secret_test() {

        // Run test from function
        secret_tests::<BasicSecret>();

    }


}
