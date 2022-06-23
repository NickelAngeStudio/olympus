/*
 * @file harpocrates/buffer.rs
 *
 * @module olympus::harpocrates
 *
 * @brief Contains buffer related functions for harpocrates.
 * 
 * @details
 * Contains buffer related functions for harpocrates.
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-06-21
 *
 * @version
 * 1.0 : 2022-06-21 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 */

#[macro_export]
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}

pub use say_hello;    // <-- the trick

/// ##### Generate a defined size buffer with defined character set.
/// Can be used to generate password, etc...
/// # Argument(s)
/// * `rng` - [`rand::Rng`] used to generate the buffer.
/// * `size` - Size of the buffer generated.
/// * `charset` - [`buffer_generator_charset`] character set flags.
/// 
/// # Example(s)
/// ```
/// // Use rand crate
/// use rand::{Rng, prelude::ThreadRng};
/// 
/// // Use harpocrates
/// use olympus::harpocrates::{self, buffer_generator_charset };
/// 
/// // Create number generator
/// let mut rng = rand::thread_rng();
///
/// // Generate buffer containing numbers and lower case of size 50
/// let buffer = harpocrates::generate_buffer(&mut rng, 50, 
///     buffer_generator_charset::LOWER_CASE | buffer_generator_charset::NUMBER);
///
/// // Print generated buffer
/// println!("Buffer = [{}]", String::from_utf8_lossy(&buffer));
/// 
/// ```
/// 
/// # Panic(s)
/// * generate_buffer() will panic if no `charset` buffer_generator_flag matches.
/// * generate_buffer() will panic if `size` == 0.
pub fn generate_buffer(rng : &mut impl rand::Rng, size : usize, charset: u8) -> Vec<u8> {

    // Make sure size generated > 0
    assert!(size > 0);

    // Make sure we use any charset.
    assert!(((charset & buffer_generator_charset::NUMBER) +
        (charset & buffer_generator_charset::LOWER_CASE) +
        (charset & buffer_generator_charset::UPPER_CASE) +
        (charset & buffer_generator_charset::SYMBOL) +
        (charset & buffer_generator_charset::UNREADABLE)) > 0);

    // Buffer to be filled
    let mut buffer: Vec<u8> = vec![0; size];

    // Vector of charset range to use
    let mut vec_charset: Vec<std::ops::RangeInclusive<u8>> = Vec::new();

    // If all characters set are used
    if charset >= 31 {
        vec_charset.push(buffer_generator_range::ALL_RANGE_0);
    } else {
        // Add number to charset
        if charset & buffer_generator_charset::NUMBER > 0 {
            vec_charset.push(buffer_generator_range::NUMBER_RANGE_0);
        }

        // Add lower case to charset
        if charset & buffer_generator_charset::LOWER_CASE > 0 {
            vec_charset.push(buffer_generator_range::LOWER_CASE_RANGE_0);
        }

        // Add upper case to charset
        if charset & buffer_generator_charset::UPPER_CASE > 0 {
            vec_charset.push(buffer_generator_range::UPPER_CASE_RANGE_0);
        }

        // Add symbol to charset
        if charset & buffer_generator_charset::SYMBOL > 0 {
            vec_charset.push(buffer_generator_range::SYMBOL_RANGE_0);
            vec_charset.push(buffer_generator_range::SYMBOL_RANGE_1);
            vec_charset.push(buffer_generator_range::SYMBOL_RANGE_2);
            vec_charset.push(buffer_generator_range::SYMBOL_RANGE_3);
        }

        // Add unreadable to charset
        if charset & buffer_generator_charset::UNREADABLE > 0 {
            vec_charset.push(buffer_generator_range::UNREADABLE_RANGE_0);
            vec_charset.push(buffer_generator_range::UNREADABLE_RANGE_1);
        } 
    }

    // Fill buffer with character sets
    for i in 0..size {
        // Select a random charset
        let cs = rng.gen_range(0..vec_charset.len());

        // Fill buffer with selected charset
        buffer[i] = rng.gen_range(vec_charset[cs].clone());
    }
    
    // Return generated buffer
    buffer
}

/// ##### Buffer generator flags used to provide character sets when generating buffer.
/// # Example(s)
/// ```
/// // Create a charset that will use numbers, lower case and symbols
/// let charset : u8 = buffer_generator_charset::NUMBER | buffer_generator_charset::LOWER_CASE | buffer_generator_charset::SYMBOL;
/// ```
pub mod buffer_generator_charset {
    /// Include number 0..9 (10 characters) when generating buffer.
    pub const NUMBER: u8 = 1;

    /// Include lower case a..z (26 characters) when generating buffer.
    pub const LOWER_CASE: u8 = 2;

    /// Include upper case A..Z (26 characters) when generating buffer.
    pub const UPPER_CASE: u8 = 4;

    /// Include symbols (!,#,$,%,...) (32 characters) when generating buffer.
    pub const SYMBOL: u8 = 8;

    /// Include unreadable symbols (NULL, TAB, BS, DEL ,...) (162 characters) when generating buffer.
    /// 
    /// Printing the buffer will show unreadable characters.
    pub const UNREADABLE: u8 = 16;

    /// Include all characters including unreadable (256 characters).
    pub const ALL: u8 = 31;

    
    
}

/// Definition of character set ranges constants
#[doc(hidden)]
pub mod buffer_generator_range {
    pub const NUMBER_RANGE_0 : std::ops::RangeInclusive<u8> = 48..=57;

    pub const LOWER_CASE_RANGE_0 : std::ops::RangeInclusive<u8> = 97..=122;

    pub const UPPER_CASE_RANGE_0 : std::ops::RangeInclusive<u8> = 65..=90;

    pub const SYMBOL_RANGE_0 : std::ops::RangeInclusive<u8> = 33..=47;
    pub const SYMBOL_RANGE_1 : std::ops::RangeInclusive<u8> = 58..=64;
    pub const SYMBOL_RANGE_2 : std::ops::RangeInclusive<u8> = 91..=96;
    pub const SYMBOL_RANGE_3 : std::ops::RangeInclusive<u8> = 123..=126;

    pub const UNREADABLE_RANGE_0 : std::ops::RangeInclusive<u8> = 0..=32;
    pub const UNREADABLE_RANGE_1 : std::ops::RangeInclusive<u8> = 127..=255;

    pub const ALL_RANGE_0 : std::ops::RangeInclusive<u8> = 0..=255;
}

 /// ##### Wipe the buffer, overwritting it with zeroes.
 /// Wipe a sensible buffer to prevent memory scan retrieval for greater security.
 /// 
 /// # Argument(s)
 /// * `buffer` - Mutable reference to vector of [`u8`] to wipe.
 /// 
 /// # Warning(s)
 /// It goes without saying that it can't be reversed.
 /// 
 /// # Example(s)
 /// ```
 /// // Import harpocrates module.
 /// use olympus::harpocrates;
 /// 
 /// // Create a u8 array
 /// let mut buffer : &mut Vec<u8> = &mut vec![80, 76, 90, 87, 73, 80, 69, 77, 69];
 /// 
 /// // Print current buffer
 /// println!("Buffer = [{}]", String::from_utf8_lossy(buffer));
 /// 
 /// // Wipe buffer
 /// harpocrates::wipe_buffer(&mut buffer);
 /// 
 /// // Print wiped buffer
 /// println!("Buffer = [{}]", String::from_utf8_lossy(buffer));
 /// ```
 pub fn wipe_buffer(buffer : &mut Vec<u8>){

    for elem in buffer.iter_mut() {
        *elem = 0;
    }

 }