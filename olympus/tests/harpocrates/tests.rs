/*
 * @file tests/harpocrates/tests.rs
 *
 * @module olympus::tests::harpocrates::tests
 *
 * @brief Contains harpocrates integration tests.
 * 
 * @details
 * Contains harpocrates integration tests.
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-07-18
 *
 * @version
 * 1.0 : 2022-07-18 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 */

use super::function::secret_test::secret_test;
use super::implementation::basic_secret::BasicSecret;

#[test]
fn basic_secret_test() {

    // Run test from function
    secret_test::<BasicSecret>();

}