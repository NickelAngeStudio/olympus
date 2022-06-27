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
pub use olympus::{harpocrates::{Secret, SecretNewOptions, generate_buffer, buffer_generator_charset, wipe_buffer}, copy_slices};

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

                // Create padded size
                let padded_size = rng.gen_range((buffer.len()/2)..buffer.len());

                for i in 0..padded_size {
                    // Conceal content 
                    if i < buffer.len() {
                        content.push(buffer[i] ^ mask[i % mask.len()]);
                    } else {
                        // Create trash
                        content.push(buffer[rng.gen_range(0..buffer.len())] ^ mask[i % mask.len()]);
                    }
                }
                

                return BasicSecret {
                    mask,
                    content
                };
            },
            olympus::harpocrates::SecretNewOptions::FromState(state) => {

                let mask  = generate_buffer(& mut rng, state.len(), buffer_generator_charset::ALL);
                let mut content = Vec::new();

                // Create padded size
                let padded_size = rng.gen_range((state.len()/2)..state.len());

                for i in 0..padded_size {
                    // Conceal content 
                    if i < state.len() {
                        content.push(state[i] ^ mask[i % mask.len()]);
                    } else {
                        // Create trash
                        content.push(state[rng.gen_range(0..state.len())] ^ mask[i % mask.len()]);
                    }
                }
                

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
        let usize_bytes =  core::mem::size_of::<usize>();
        let capacity = usize_bytes + self.mask.len() + self.content.len();
        let mut state: Vec<u8> = vec![0; capacity];

        // Copy content into state vec
        copy_slices!(&mut state, 0, &self.mask.len().to_ne_bytes(), &self.mask, &self.content);

        // Return state
        state
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


#[test]
fn get_secret_state() {

    let mut rng = rand::thread_rng();
    let mut buffer = generate_buffer(&mut rng, 50, buffer_generator_charset::NUMBER);
    let bs = BasicSecret::new(SecretNewOptions::FromBuffer(&buffer));
    wipe_buffer(&mut buffer);

    let state = bs.get_state();

    print!("\nBuffer = ");

    // Print each element of vector
    for elem in state.iter() {
        print!("{}", elem);
    }

    print!("\n");

}