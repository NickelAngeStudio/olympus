/*
 * @file harpocrates/test/buffer_test.rs
 *
 * @module olympus::harpocrates::test
 *
 * @brief Contains tests for buffer functions
 * 
 * @details
 * Contains tests for buffer functions
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-06-22
 *
 * @version
 * 1.0 : 2022-06-22 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 */

use std::time::{Duration, Instant};

use rand::{Rng, prelude::ThreadRng};

use crate::harpocrates::{self, buffer_generator_charset, buffer::buffer_generator_range, wipe_buffer};

// Size of buffer for tests (except Stress)
static BUFFER_SIZE: usize = 255;

// Stress test duration in seconds (10 mins)
static STRESS_DURATION: Duration = Duration::from_secs(600); 

// Max buffer stress size set to 4mb, which is the typical 3d model file. https://www.google.com/search?q=typical+3d+model+file+size
static STRESS_MAX_SIZE : usize = 4*1024*1024;


/******************
* GENERATE_BUFFER *
******************/
#[test]
#[should_panic]
// Generate a buffer with size 0
fn generate_buffer_size_0(){

    let mut rng = rand::thread_rng();

    let buffer = harpocrates::generate_buffer(&mut rng, 0, buffer_generator_charset::ALL);

    println!("Buffer = [{}]", String::from_utf8_lossy(&buffer));

}

#[test]
#[should_panic]
// Generate a buffer without charset
fn generate_buffer_no_charset(){

    let mut rng = rand::thread_rng();

    let buffer = harpocrates::generate_buffer(&mut rng, BUFFER_SIZE, 0);

    println!("Buffer = [{}]", String::from_utf8_lossy(&buffer));

}

#[test]
// Generate a buffer with each charset and test set validity
fn generate_and_test_characters_set(){

    let mut rng = rand::thread_rng();

    test_character_set_validity(&mut rng, buffer_generator_charset::NUMBER, BUFFER_SIZE, true);
    test_character_set_validity(&mut rng,buffer_generator_charset::LOWER_CASE, BUFFER_SIZE, true);
    test_character_set_validity(&mut rng,buffer_generator_charset::UPPER_CASE, BUFFER_SIZE, true);
    test_character_set_validity(&mut rng,buffer_generator_charset::SYMBOL, BUFFER_SIZE, true);
    test_character_set_validity(&mut rng,buffer_generator_charset::UNREADABLE, BUFFER_SIZE, true);
    test_character_set_validity(&mut rng,buffer_generator_charset::ALL, BUFFER_SIZE, true);


}

#[test]
#[ignore]
// Stress test buffer for STRESS_DURATION. Can take long time.
fn generate_buffer_stress_test() {

    let mut rng = rand::thread_rng();
    let mut stress_loop = 0;
    let started = Instant::now();

    while Instant::now() - started <= STRESS_DURATION {    
        stress_loop += 1;

        // Generate buffer size
        let size = rng.gen_range(0..=STRESS_MAX_SIZE);
        
        // Generate buffer charset
        let charset = generate_charset(&mut rng);

        // Print loop # and variables
        println!("Stress #{} | Buffer size={}/{} | Charset={} | Remaining time : {:?}...", stress_loop, size, STRESS_MAX_SIZE,
            charset, STRESS_DURATION - (Instant::now() - started));

        // Test character set validity
        test_character_set_validity(&mut rng, charset, size, false);
    }
}


/**************
* WIPE_BUFFER *
**************/
#[test]
fn wipe_generated_buffer() {

    let mut rng = rand::thread_rng();
    let charset = buffer_generator_charset::NUMBER | buffer_generator_charset::LOWER_CASE | 
                buffer_generator_charset::UPPER_CASE | buffer_generator_charset::SYMBOL;

    let mut buffer = harpocrates::generate_buffer(&mut rng, BUFFER_SIZE, charset);

    println!("Buffer before wipe = {}", String::from_utf8_lossy(&buffer));

    wipe_buffer(&mut buffer);

    print!("\nBuffer after wipe = ");

    // Print each element of vector
    for elem in buffer.iter() {
        print!("{}", elem);
    }

    print!("\n");
}

/***************************
 * FUNCTIONS USED IN TESTS *
 **************************/
/// Generate a valid random charset
fn generate_charset(rng : &mut ThreadRng) -> u8{

    let mut charset:u8 = 0;


    // While charset is invalid, generate a charset
    while charset == 0 {

        // Pick numbers for charset generation 
        let number = rng.gen_range(0..=1);
        let lower_case = rng.gen_range(0..=1);
        let upper_case = rng.gen_range(0..=1);
        let symbol = rng.gen_range(0..=1);
        let unreadable = rng.gen_range(0..=1);

        // Lower chance to get all (1 out of 10)
        let all = rng.gen_range(0..=10);

        if number == 1 {
            charset = charset | buffer_generator_charset::NUMBER;
        }

        if lower_case == 1 {
            charset = charset | buffer_generator_charset::LOWER_CASE;
        }

        if upper_case == 1 {
            charset = charset | buffer_generator_charset::UPPER_CASE;
        }

        if symbol == 1 {
            charset = charset | buffer_generator_charset::SYMBOL;
        }

        if unreadable == 1 {
            charset = charset | buffer_generator_charset::UNREADABLE;
        }

        if all == 1 {
            charset = charset | buffer_generator_charset::ALL;
        }
    }

    charset
}

/// Will test a specific charset and panic if invalid 
fn test_character_set_validity(rng : &mut ThreadRng, charset : u8, size: usize, print_buf : bool){

    let buffer = harpocrates::generate_buffer(rng, size, charset);

    // Print buffer if print_buf
    if print_buf {
        println!("Buffer = {}", String::from_utf8_lossy(&buffer));
    }

    // Make sure that buffer contains desired charset
    assert!(buffer_charset_valid(&buffer, charset));

}

/// Verify that a buffer charset is in range
fn buffer_charset_valid(buffer: &Vec<u8>, charset : u8) -> bool {

    // Start with result as valid
    let mut result = true;

    // Vector of accepted characters
    let mut vec_char: Vec<bool> = Vec::with_capacity(256);

    // Init the vector as all characters invalids
    for _ in 0..256 {
        vec_char.push(false);
    }

    // Fill vec_char to see if it is a valid character
    if charset >= 31 {
        for i in buffer_generator_range::ALL_RANGE_0 {
            vec_char[i as usize] = true;
        }
    } else {
        if charset & buffer_generator_charset::NUMBER > 0 {
            for i in buffer_generator_range::NUMBER_RANGE_0 {
                vec_char[i as usize] = true;
            }
        }

        // Add lower case to charset
        if charset & buffer_generator_charset::LOWER_CASE > 0 {
            for i in buffer_generator_range::LOWER_CASE_RANGE_0 {
                vec_char[i as usize] = true;
            }
        }

        // Add upper case to charset
        if charset & buffer_generator_charset::UPPER_CASE > 0 {
            for i in buffer_generator_range::UPPER_CASE_RANGE_0 {
                vec_char[i as usize] = true;
            }
        }

        // Add symbol to charset
        if charset & buffer_generator_charset::SYMBOL > 0 {
            for i in buffer_generator_range::SYMBOL_RANGE_0 {
                vec_char[i as usize] = true;
            }
            for i in buffer_generator_range::SYMBOL_RANGE_1 {
                vec_char[i as usize] = true;
            }
            for i in buffer_generator_range::SYMBOL_RANGE_2 {
                vec_char[i as usize] = true;
            }
            for i in buffer_generator_range::SYMBOL_RANGE_3 {
                vec_char[i as usize] = true;
            }
        }

        // Add unreadable to charset
        if charset & buffer_generator_charset::UNREADABLE > 0 {
            for i in buffer_generator_range::UNREADABLE_RANGE_0 {
                vec_char[i as usize] = true;
            }
            for i in buffer_generator_range::UNREADABLE_RANGE_1 {
                vec_char[i as usize] = true;
            }
        } 
    }

    // Verify if character is valid. If ANY character is invalid, result will become false.
    for i in 0..buffer.len() {
        result = result && vec_char[buffer[i] as usize];
    }


    result
}