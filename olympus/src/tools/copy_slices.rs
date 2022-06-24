/*
 * @file tools/copy_slices.rs
 *
 * @module olympus::tools
 *
 * @brief Macro used to easily fill a slice from other slice without worrying for size.
 * 
 * @details
 * Macro used to easily fill a slice from other slice without worrying for size.
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-06-24 - Happy birthday Quebec
 *
 * @version
 * 1.0 : 2022-06-24 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 */

/// ##### Macro used to copy multiple slice into one slice without worrying for the matching length.
/// Macro used to copy multiple [`slice`] into one [`slice`] without worrying for the matching length.
/// 
/// Copying [`slice`] via [`copy_from_slice`](https://doc.rust-lang.org/std/primitive.slice.html#method.copy_from_slice)
/// need both destination and source length to match. We usually cope with that
/// using `dst[0..src0.len()].copy_from_slice(src0)`, then `dst[src0.len()..src0.len() + src1.len].copy_from_slice(src1)`
/// when we want to copy more [`slice`], making the code more complicated and more prone to error.
/// 
/// Enter macro [`copy_slices!`] which will copy all sources [`slice`] sequentially as long as destination [`slice`] can
/// contain them all.
/// 
/// # Argument(s)
/// * `dst` - Mutable destination [`slice`] where the sources will be copied. Length need to be able to contains ALL sources.
/// * `index` - Position in the `dst` [`slice`] where to copy the `src` [`slice`]. Use 0 to start at the beginning.
/// * `[src]` - 1..N sources [`slice`] to be copied sequentially into `dst`.
/// 
/// # Example(s)
/// ```
/// // Creating 1 destination and 2 sources
/// let mut x = vec!(0,0,0,0,0,0);
/// let y = vec!(1,2,3,4);
/// let z = vec!(5,6);
/// 
/// // Will copy y and z starting at position 0 of x. x will now be [1,2,3,4,5,6]
/// copy_slices!(&mut x, 0, &y, &z);
/// ```
/// # Panic(s)
/// * Will panic! if using less than 3 arguments.
/// * Will panic! if `dst` length is smaller than all `src` length combined.
#[macro_export]
macro_rules! copy_slices {

    // Panic since no arguments
    () => {
        panic!("copy_slices needs at least 3 arguments (missing : dst, index, src)!");
    };
    // Panic since only destination
    ($dst:expr) => {
        panic!("copy_slices needs at least 3 arguments (missing : index, src)!");
    };
    // Panic since only destination and index (missing sources)
    ($dst:expr, $index:expr) => {
        panic!("copy_slices needs at least 3 arguments (missing : src)!");
    };
    // Copy slice at position index
    ($dst:expr, $index:expr, $src:expr) => {
        $dst[$index..($index + $src.len())].copy_from_slice($src);
    };
    // Variadic copy slice at position index
    ($dst:expr, $index:expr, $src:expr, $($extra:expr)*) => {
        $dst[$index..($index + $src.len())].copy_from_slice($src);
        copy_slices!($dst, $index + $src.len(), $($extra)*);
    };

}


#[cfg(test)]
mod test {
    /// Test trying with 0 argument
    #[test]
    #[should_panic]
    fn copy_slices_0_args() {

        copy_slices!();

    }

    /// Test trying with 1 argument
    #[test]
    #[should_panic]
    fn copy_slices_1_args() {

        let mut _x = vec!(0,0,0,0,0,0);
        copy_slices!(&mut x);

    }

    /// Test trying with 2 arguments
    #[test]
    #[should_panic]
    fn copy_slices_2_args() {

        let mut _x = vec!(0,0,0,0,0,0);
        copy_slices!(&mut x, 0);

    }

    
    /// Test to copy 2 slices in 1
    #[test]
    fn test_copy_slices() {

        let mut x = vec!(0,0,0,0,0,0);
        let y = vec!(1,2,3,4);
        let z = vec!(5,6);
        

        print!("\nBefore = ");

        // Print each element of vector
        for elem in x.iter() {
            print!("{}", elem);
        }

        print!("\n");

        copy_slices!(&mut x, 0, &y, &z);


        print!("\nAfter = ");

        // Print each element of vector
        for elem in x.iter() {
            print!("{}", elem);
        }

        print!("\n");
    }

    // TODO: Stress test
}