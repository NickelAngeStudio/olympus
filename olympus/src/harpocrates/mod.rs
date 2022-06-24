/*
 * @file harpocrates/mod.rs
 *
 * @module olympus::harpocrates
 *
 * @brief Contains header of API interface for harpocrates components.
 * 
 * @details
 * Contains header of API interface for harpocrates components.
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


  /// # Re-export of functions for Public API
#[doc(inline)]
pub use buffer::generate_buffer as generate_buffer;
pub use buffer::wipe_buffer as wipe_buffer;
pub use buffer::buffer_generator_charset as buffer_generator_charset;


// Buffer related functions
#[doc(hidden)]
mod buffer;

 /// ##### Abstraction used for buffer encryption.
 /// 
 /// Cypher is used to create an encryption abstraction.
pub trait Cypher {
    /// # Description
    /// Encrypt a buffer according to the seek cursor position.
    /// 
    /// # Argument(s)
    /// * `buffer` - Mutable reference to vector of [`u8`] to encrypt.
    /// * `seek_position` - Position of the seek cursor (AKA tell).
    /// 
    /// # Warning(s)
    /// Referenced buffer WILL be overwritten!
    fn encrypt(&self, buffer : &mut Vec<u8>, seek_position : usize);
}


 /// ##### Abstraction used for buffer decryption.
 /// 
 /// Decypher is used to create a decryption abstraction.
pub trait Decypher {
    /// # Description
    /// Decrypt a buffer according to the seek cursor position.
    /// 
    /// # Argument(s)
    /// * `buffer` - Mutable reference to vector of [`u8`] to decrypt.
    /// * `seek_position` - Position of the seek cursor (AKA tell).
    /// 
    /// # Warning(s)
    /// Referenced buffer WILL be overwritten!
    fn decrypt(&self, buffer : &mut Vec<u8>, seek_position : usize);
}

 /// ##### Abstraction used to conceal a passphrase or secret.
 /// 
 /// Abstraction used to conceal a passphrase (or secret) to prevent easy retrieval via memory scan. 
 /// 
 /// Also provide a way to save the secret state for storage without revealing it.
 /// 
 /// Secret implementation shouldn't keep the passphrase length nor implement an easy way
 /// to retrieve the passphrase for greater security.
 /// 
 /// The only way to retrieve the passphrase is by using [`Secret::get_secret_at(index: usize)`](trait.Secret.html#tymethod.get_secret_at)
 /// and retrieve it character by character. Index overflow should return fixed junk and not
 /// panic to prevent passphrase length retrieval.
 /// # Diagram
 /// TODO: add SVG workflow
pub trait Secret {
    /// # Description
    /// Create secret according to [`SecretNewOptions`] enum
    /// # Argument(s)
    /// `new_option` - [`SecretNewOptions`] used to create the Secret
    /// # Return
    /// New Secret created
    fn new(new_option : SecretNewOptions) -> Self;

    /// # Description
    /// Get the secret u8 char at index.
    /// # Argument(s)
    /// `index` - Index to retrieve the secret char.
    /// # Return
    /// Copy of secret u8 char at index or junk if index overflow.
    /// # Panic(s)
    /// get_secret_at() must NOT panic at overflow to prevent passphrase length retrieval.
    fn get_secret_at(&self, index: usize) -> u8;

    /// # Description
    /// Get the current state of the Secret that allow to rebuild the secret without the passphrase itself.
    /// The state could be then saved to a file or added directly to the code.
    /// # Return
    /// Secret state in a u8 vector.
    fn get_state(&self) -> Vec<u8>;

    /// # Description
    /// Create an independant clone of the secret using state.
    /// 
    /// Mostly used for multi-threaded secret access without mutex.
    /// # Return
    /// Totally independant clone of secret.
    fn clone(&self) -> Self where Self: Sized {
        let state= self.get_state();
        Self::new(SecretNewOptions::FromState(&state)) 
    }

    /// # Description
    /// Clear all the Secret data, making it impossible to retrieve the state.
    /// 
    /// Should be called when [`Drop`].
    fn clear(&mut self); 
}

/// ##### Enum of possible options when creating a new secret via [`Secret::new()`](trait.Secret.html#tymethod.new).
pub enum SecretNewOptions<'a> {
    /// # Description
    /// Create the new secret from a buffer containing the information to conceal.
    /// 
    /// # Argument(s)
    /// * `&'a Vec<u8>` - Reference to vector of [`u8`] containing the information to conceal.
    FromBuffer(&'a Vec<u8>),

    /// # Description
    /// Create the new secret from a buffer containing a previous state of the secret.
    /// 
    /// # Argument(s)
    /// * `&'a Vec<u8>` - Reference to vector of [`u8`] containing the state.
    FromState(&'a Vec<u8>),
}

 // harpocrates module tests
#[cfg(test)]
mod test;