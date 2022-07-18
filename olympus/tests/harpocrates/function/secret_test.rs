/*
 * @file tests/harpocrates/function/secret_test.rs
 *
 * @module olympus::tests::harpocrates::integration
 *
 * @brief Contains test function to test Secret trait integrations.
 * 
 * @details
 * Contains test function to test Secret trait integrations.
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

use olympus::harpocrates::{Secret, SecretNewOptions};
use tampon::{compare_buffers, generate_buffer, buffer_generator_charset::{ LOWER_CASE, UPPER_CASE, NUMBER }};


// Size of secret
const SECRET_SIZE: usize = 50;

// Total count of tests
const TEST_COUNT:usize = 13;


// Test a Secret implementation.
pub fn secret_test<S : Secret>() {

    // Generate secret buffer
    let secret_buffer = generate_secret_buffer();

    // Create secret from buffer
    let mut secret = create_secret_from_buffer::<S>(&secret_buffer);

    // Retrieve secret and compare
    retrieve_secret(&secret, SECRET_SIZE, &secret_buffer, 3);

    // Fetch fictive datas
    fetch_fictive_datas(&secret, 4);

    // Test get_state
    let state = get_secret_state(&secret, 5);

    // Recreate from state
    recreate_from_state::<S>(&state, &secret_buffer, 6);


    // Clone secret
    clone_secret(&secret, &secret_buffer, 9);
    
    // Test clear
    clear_secret(&mut secret, &secret_buffer, SECRET_SIZE, 12);


    // Secret implementation passed!
    println!("\nYour Secret implementation passed the tests!\n");
}

// Generate the secret to hide into Secret
fn generate_secret_buffer() -> Vec<u8> {
    // Write step
    println!("\n*** Step 1 of {} *** Generating secret buffer...", TEST_COUNT);

    // RNG used to generate buffer
    let mut rng = rand::thread_rng();
    // Generate secret buffer
    let secret_buffer = generate_buffer(&mut rng, SECRET_SIZE, LOWER_CASE | UPPER_CASE | NUMBER);

    // Print secret_buffer
    println!("Secret buffer={:?}", secret_buffer);

    // Return secret_buffer
    secret_buffer
}

fn create_secret_from_buffer<S : Secret>(secret_buffer: &Vec<u8>) -> S {
    // Write step
    println!("\n*** Step 2 of {} *** Creating Secret from buffer...", TEST_COUNT);

    // Create and return secret
    S::new(SecretNewOptions::FromBuffer(&secret_buffer))
}

// Retrieve secret from Secret
fn retrieve_secret<S : Secret>(secret : &S, size : usize, secret_buffer: &Vec<u8>, step:usize) {

    // Write step
    println!("\n*** Step {} of {} *** Retrieving secret from Secret...", step, TEST_COUNT);
    
    let mut retrieved: Vec<u8> = vec![0; size];

    for i in 0..size {
        retrieved[i] = secret.get_secret_at(i);
    }

    println!("Original={:?}\nRetrieve={:?}", secret_buffer, retrieved);

    // Both retrieve and original secret should be identical.
    assert!(compare_buffers(&secret_buffer, &retrieved) == 0, "Original secret and retrieved secret are different!");

    println!("Both buffers are identical");
}

// Fetch fictives data that shouldn't panic!
fn fetch_fictive_datas<S : Secret>(secret : &S, step:usize) {

    // Write step
    println!("\n*** Step {} of {} *** Fetching fictives data from Secret...", step, TEST_COUNT);

    print!("Fictives = ");
    for i in SECRET_SIZE..(SECRET_SIZE * 5) {
        print!("{} ", secret.get_secret_at(i));
    }
    print!("\n");

}

// Get state of secret
fn get_secret_state<S : Secret>(secret : &S, step:usize) -> Vec<u8> {

    // Write step
    println!("\n*** Step {} of {} *** Fetching state from Secret...", step, TEST_COUNT);

    let state = secret.get_state();

    // Make sure state length > 0
    assert!(state.len() > 0, "State length should be greater than 0.");

    // Return state
    state
}

// Recreate secret from state and test retrieve + fictives 
fn recreate_from_state<S : Secret>(state:&Vec<u8>, secret_buffer: &Vec<u8>, step:usize) {

    // Write step
    println!("\n*** Step {} of {} *** Recreating Secret from state...", step, TEST_COUNT);

    // Recreate secret from state
    let secret_from_state = S::new(SecretNewOptions::FromState(&state));

    // Retrieve secret and compare
    retrieve_secret(&secret_from_state, SECRET_SIZE, &secret_buffer, step + 1);

    // Fetch fictive datas
    fetch_fictive_datas(&secret_from_state, step + 2);
}

// Clone secret and test it
fn clone_secret<S : Secret>(secret : &S, secret_buffer: &Vec<u8>, step:usize){

    // Write step
    println!("\n*** Step {} of {} *** Cloning Secret...", step, TEST_COUNT);

    // Clone secret
    let secret_clone = secret.clone();

    // Retrieve secret and compare
    retrieve_secret(&secret_clone, SECRET_SIZE, &secret_buffer, step + 1);

    // Fetch fictive datas
    fetch_fictive_datas(&secret_clone, step + 2);
}

// Clear secret and compare retrieved VS original
fn clear_secret<S : Secret>(secret : &mut S, secret_buffer: &Vec<u8>, size : usize, step:usize) {

    // Write step
    println!("\n*** Step {} of {} *** Clearing Secret...", step, TEST_COUNT);

    // Clear secret
    secret.clear();

    // Fetch fictive datas after clear (should not panic)
    fetch_fictive_datas(secret, step + 1);

    // Retrieve after clear
    let mut retrieved: Vec<u8> = vec![0; size];
    for i in 0..size {
        retrieved[i] = secret.get_secret_at(i);
    }

    // Compare should have different result and never be 0
    assert!(compare_buffers(&secret_buffer, &retrieved) > 0, "Original secret and retrieved secret should be different after clear!");

    println!("Secret was cleared successfully!");


}