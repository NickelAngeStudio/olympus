/*
 * @file tests/harpocrates/function.rs
 *
 * @module olympus::tests::function
 *
 * @brief Contains reusable functions used for integration tests.
 * 
 * @details
 * Contains reusable functions used for integration tests.
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

use std::vec;

use olympus::harpocrates::{Secret, generate_buffer, buffer_generator_charset::{LOWER_CASE, UPPER_CASE, NUMBER}, SecretNewOptions};


// Size of secret
const SECRET_SIZE: usize = 50;

/***************
* SECRET TRAIT *
***************/
// Test a Secret implementation.
pub fn secret_tests<S : Secret>() {

    // RNG used to generate buffer
    let mut rng = rand::thread_rng();

    // Generate secret buffer
    let secret_buffer = generate_buffer(&mut rng, SECRET_SIZE, LOWER_CASE | UPPER_CASE | NUMBER);

    // Create secret to test while testing New::SecretNewOptions::FromBuffer
    let secret = S::new(SecretNewOptions::FromBuffer(&secret_buffer));


    // Test get_secret_at to retrieve secret
    let mut retrieved: Vec<u8> = vec![0; SECRET_SIZE];

    for i in 0..SECRET_SIZE {
        retrieved[i] = secret.get_secret_at(i);
    }

    println!("Secret = {:?}", secret_buffer);
    println!("Retrieved = {:?}", secret_buffer);

    // Compare buffers and panic! if difference is NOT 0
    assert!(compare_buffers(&secret_buffer, &retrieved) == 0);

    // Test extra characters that shouldn't panic.
    print!("Fictives = ");
    for i in SECRET_SIZE..(SECRET_SIZE * 5) {
        print!("{} ", secret.get_secret_at(i));
    }
    print!("\n");

    // Test get_state
    let state = secret.get_state();
    println!("State = {:?}", state);



    // TODO: Recreate from state
    let secret_from_state = S::new(SecretNewOptions::FromState(&state));

    let mut retrieved: Vec<u8> = vec![0; SECRET_SIZE];

    for i in 0..SECRET_SIZE {
        retrieved[i] = secret.get_secret_at(i);
    }

    // TODO:Test clone


    // TODO:Test clear





}




/***************
* CYPHER TRAIT *
***************/


/*****************
* DECYPHER TRAIT *
*****************/


/****************
* FUNCTION USED *
****************/
// Compare 2 buffers and return the difference. (equal would be 0)
fn compare_buffers(b1 : &Vec<u8>,  b2 : &Vec<u8>) -> usize {
        
    let mut diff:usize = 0;
    let mut size = 0;

    if b1.len() > b2.len() {
        size = b1.len() - b2.len();
        diff = b1.len() - b2.len();
    } else {
        size = b2.len() - b1.len();
        diff = b2.len() - b1.len();
    }
    
    for i in 0..size {

        if b1[i] > b2[i] {
            diff = diff + (b1[i] - b2[i]) as usize;
        } else {
            diff = diff + (b2[i] - b1[i]) as usize;
        }
    }
     
    diff

}