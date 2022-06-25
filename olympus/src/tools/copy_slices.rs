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
/// * `[src]` - 1..n sources [`slice`] to be copied sequentially into `dst`.
/// 
/// # Example(s)
/// ```
/// // Import macro
/// use olympus::copy_slices;
/// 
/// // Creating 1 destination and 2 sources
/// let mut dst = vec!(0,0,0,0,0,0);
/// let src0 = vec!(1,2,3,4);
/// let src1 = vec!(5,6);
/// 
/// // Will copy `src0` and `src1` starting at position 0 of `dst`. `dst` will now be [1,2,3,4,5,6].
/// copy_slices!(&mut dst, 0, &src0, &src1);
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
    ($dst:expr, $index:expr, $src:expr, $($srcs:expr), +) => {
        $dst[$index..($index + $src.len())].copy_from_slice($src);
        copy_slices!($dst, $index + $src.len(), $($srcs), +);
    };

}


#[cfg(test)]
mod test {
    use std::time::{Duration, Instant};

    use rand::{Rng, prelude::ThreadRng};

    // Max elements to copy in src for stress test (limited to 4 mb)
    static MAX_STRESS_SRC_ELEM: i32 = 4*1024*1024;

    // Maximum value of each source elements
    static MAX_STRESS_SRC_VALUE: i32 =  std::i32::MAX;

    // Stress test duration in seconds (10 mins)
    static STRESS_DURATION: Duration = Duration::from_secs(600); 

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
    fn copy_2_slices() {

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

    /// Test to copy 2 slices in 1 but with dest smaller in size
    #[test]
    #[should_panic]
    fn copy_2_slices_missing_space() {

        let mut x = vec!(0);
        let y = vec!(1,2,3,4);
        let z = vec!(5,6);

        copy_slices!(&mut x, 0, &y, &z);
    }

    /// Test to copy 2 empty slices
    #[test]
    fn copy_2_empty_slices() {

        let mut x = vec!(0);
        let y = Vec::new();
        let z = Vec::new();

        copy_slices!(&mut x, 0, &y, &z);
    }

    // Stress test copy slices to test stability, large slices and slices content (any differences will trigger a panic)
    #[test]
    #[ignore]
    fn copy_slices_stress() {

        // Destination Vector
        let mut dest: Vec<i32> = Vec::new();

        // Vector of sources
        let mut srcs : Vec<Vec<i32>> = Vec::new();

        // Rng for random numbers
        let mut rng = rand::thread_rng();

        let mut stress_loop = 0;
        let started = Instant::now();

        while Instant::now() - started <= STRESS_DURATION {    
            stress_loop += 1; 
            
            // Print test header and remaining time
            println!("STRESS #{} | Remaining time : {:?}...", stress_loop, if  Instant::now() - started < STRESS_DURATION {
                STRESS_DURATION - (Instant::now() - started)
            } else {
                Duration::from_secs(0)
            });


            // Copy 1 slices
            let total_size = generate_srcs(&mut rng, &mut srcs, 1, MAX_STRESS_SRC_ELEM as usize);
            resize_destination(&mut dest, total_size);
            copy_slices!(&mut dest, 0, &srcs[0]);
            
            let diff = compare_dest_srcs(&dest, &srcs);
            println!("    1 slice | Total size={} | Diff={}", total_size, diff);
            assert!(diff == 0);
            
            
            // Copy 2 slices
            let total_size = generate_srcs(&mut rng, &mut srcs, 2, MAX_STRESS_SRC_ELEM as usize);
            resize_destination(&mut dest, total_size);
            copy_slices!(&mut dest, 0, &srcs[0], &srcs[1]);

            let diff = compare_dest_srcs(&dest, &srcs);
            println!("    2 slices | Total size={} | Diff={}", total_size, diff);
            assert!(diff == 0);

            // Copy 3 slices
            let total_size = generate_srcs(&mut rng, &mut srcs, 3, MAX_STRESS_SRC_ELEM as usize);
            resize_destination(&mut dest, total_size);
            copy_slices!(&mut dest, 0, &srcs[0], &srcs[1], &srcs[2]);

            let diff = compare_dest_srcs(&dest, &srcs);
            println!("    3 slices | Total size={} | Diff={}", total_size, diff);
            assert!(diff == 0);

            // Copy 100 slices
            let total_size = generate_srcs(&mut rng, &mut srcs, 100, 65535);
            resize_destination(&mut dest, total_size);
            // Yup hardcoded, bc macro
            copy_slices!(&mut dest, 0, &srcs[0], &srcs[1], &srcs[2], &srcs[3], &srcs[4], &srcs[5], &srcs[6], &srcs[7], &srcs[8], &srcs[9],
                &srcs[10], &srcs[11], &srcs[12], &srcs[13], &srcs[14], &srcs[15], &srcs[16], &srcs[17], &srcs[18], &srcs[19],
                &srcs[20], &srcs[21], &srcs[22], &srcs[23], &srcs[24], &srcs[25], &srcs[26], &srcs[27], &srcs[28], &srcs[29],
                &srcs[30], &srcs[31], &srcs[32], &srcs[33], &srcs[34], &srcs[35], &srcs[36], &srcs[37], &srcs[38], &srcs[39],
                &srcs[40], &srcs[41], &srcs[42], &srcs[43], &srcs[44], &srcs[45], &srcs[46], &srcs[47], &srcs[48], &srcs[49],
                &srcs[50], &srcs[51], &srcs[52], &srcs[53], &srcs[54], &srcs[55], &srcs[56], &srcs[57], &srcs[58], &srcs[59],
                &srcs[60], &srcs[61], &srcs[62], &srcs[63], &srcs[64], &srcs[65], &srcs[66], &srcs[67], &srcs[68], &srcs[69],
                &srcs[70], &srcs[71], &srcs[72], &srcs[73], &srcs[74], &srcs[75], &srcs[76], &srcs[77], &srcs[78], &srcs[79],
                &srcs[80], &srcs[81], &srcs[82], &srcs[83], &srcs[84], &srcs[85], &srcs[86], &srcs[87], &srcs[88], &srcs[89],
                &srcs[90], &srcs[91], &srcs[92], &srcs[93], &srcs[94], &srcs[95], &srcs[96], &srcs[97], &srcs[98], &srcs[99]);

                let diff = compare_dest_srcs(&dest, &srcs);
                println!("    100 slices | Total size={} | Diff={}\n", total_size, diff);
                assert!(diff == 0);
        }
    }

    /***************************
     * FUNCTIONS USED IN TESTS *
     **************************/
    // Generate slices for sources and return total size
    fn generate_srcs(rng: &mut ThreadRng, srcs : &mut Vec<Vec<i32>>, count:usize, elem_size:usize) -> usize{
        
        // Clear sources
        srcs.clear();

        let mut total_size: usize = 0;

        for _ in 0..count {

            let size: usize = rng.gen_range(0..elem_size) as usize;
            total_size += size;

            let mut src : Vec<i32> = Vec::new();

            // Generate values for source
            for _ in 0..size {
                src.push(rng.gen_range(0..MAX_STRESS_SRC_VALUE));
            }

            // Push source to sources
            srcs.push(src);
        }


        total_size
    }

    // Resize destination with new size and fill it with 0
    fn resize_destination(dest : &mut Vec<i32>, size:usize){

        dest.clear();

        for _ in 0..size {
            dest.push(0);
        }
    }

    // Compare destination content vs sources and return the difference. (equal would be 0)
    fn compare_dest_srcs(dest : &Vec<i32>,  srcs : &Vec<Vec<i32>>) -> usize {
        
        let mut diff:usize = 0;
        let mut dest_index = 0;

        for src in srcs.iter() {
            for elem in src.iter() {
                diff = diff + (dest[dest_index] - elem).abs() as usize;
                dest_index += 1;
            }
        }

         
        diff

    }
    
}