/*
 * @file examples/harpocrates_basic_secret/basic_secret.rs
 *
 * @module olympus::examples::harpocrates_basic_secret
 *
 * @brief Basic Secret trait implementation.
 * 
 * @details
 * Basic Secret trait implementation.
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-06-25
 *
 * @version
 * 1.0 : 2022-06-25 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 */


use rand::{ Rng};
pub use olympus::{harpocrates::{Secret, SecretNewOptions }};
use tampon::{buffer, deserialize, generate_buffer, buffer_generator_charset, wipe_buffer};

 /// Basic secret struct used to conceal content.
 /// 
 /// Not really complex nor hard to crack but always O(1) complexity.
 pub struct BasicSecret {
    // Mask used to conceal content
    mask : Vec<u8>,

    // Concealed content
    content : Vec<u8>,
 }

 impl BasicSecret {
    

 }

impl Secret for BasicSecret {
    fn new(new_option : olympus::harpocrates::SecretNewOptions) -> Self {
        
        let mut rng = rand::thread_rng();

        match new_option {
            olympus::harpocrates::SecretNewOptions::FromBuffer(buffer) => {
                
                let mask  = generate_buffer(& mut rng, buffer.len(), buffer_generator_charset::ALL);
                let mut content = Vec::new();

                // Capture and mask content
                for i in 0..buffer.len() {
                    content.push(buffer[i] ^ mask[i % mask.len()]);
                }

                // Create extra padded trash
                let padded_size = rng.gen_range((buffer.len()/2)..buffer.len());
                for i in 0..padded_size {
                    // Create trash
                    content.push(buffer[rng.gen_range(0..buffer.len())] ^ mask[i % mask.len()]);
                }

                return BasicSecret {
                    mask,
                    content
                };
            },
            olympus::harpocrates::SecretNewOptions::FromState(state) => {

                // Use tampon::deserialize! macro to retrieve mask and content
                deserialize!(state, [mask, content]:u8);

                return BasicSecret {
                    mask,
                    content
                };
            },
        }
    }

    

    fn get_secret_at(&self, index: usize) -> u8 {
        
        if index < self.content.len() {
            return self.mask[index % self.mask.len()] ^ self.content[index];

        } else {
            let mut rng = rand::thread_rng();
            return rng.gen_range(0..=255);
        }

    }

    fn get_state(&self) -> Vec<u8> {
        // Use tampon::buffer! macro to generate state
        buffer!([self.mask, self.content]:u8)
    }

    fn clear(&mut self) {
        
        wipe_buffer(&mut self.mask);
        wipe_buffer(&mut self.content);

        self.mask.clear();
        self.content.clear();

    }

    fn clone(&self) -> Self where Self: Sized {
        let state= self.get_state();
        Self::new(olympus::harpocrates::SecretNewOptions::FromState(&state)) 
    }
}