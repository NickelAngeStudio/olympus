/*
 * @file hercules/test_hercules.rs
 *
 * @module olympus::hercules
 *
 * @brief Contains unit tests for Hercules struct.
 * 
 * @details
 * Contains unit tests for Hercules struct such as :
 * - No workers
 * - 1 Workers
 * - Logical core count workers
 * - Stress test
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-06-10
 *
 * @version
 * 1.0 : 2022-06-10 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 * Test waiting
 */

use crate::Hercules;
use crate::WorkOrder;
use crate::tools;

// To calculate difference in some test(s)
use std::time::{Duration, Instant};

// Count of labor used to test
static LABOUR_COUNT: u32 = 20;

// Simple function to be executed
fn calc_fn() {

    let mut _c:u32 = 0;
    for i in 0..65535 {
        _c += i;
    }

}

#[test]
#[should_panic]
/// Test that Hercules CAN'T be created without workers (size == 0)
fn create_hercules_no_worker() {

    let _hercules = Hercules::new(0);

}


#[test]
/// Creating with only 1 thread and pushing LABOUR_COUNT labour
fn create_hercules_with_one_worker() {

    let hercules = Hercules::new(1);
    let wo = WorkOrder::new(Some(&hercules));

    for _i in 0..LABOUR_COUNT {
        wo.add_labour(move || { calc_fn(); });
        //hercules.push_labour(move || { calc_fn(); });
    }

    // Waiting
    println!("Waiting");
    wo.wait(Some(Duration::from_secs(5)));
    //hercules.wait(Some(Duration::from_secs(5)));
}

#[test]
/// Creating with logical core count and pushing LABOUR_COUNT labour
fn create_hercules_with_logical_core_count_worker() {

    let hercules = Hercules::new(tools::get_logical_core_count());

    for _i in 0..LABOUR_COUNT {
        hercules.push_labour(move || { calc_fn(); });
    }

    //hercules.wait(Some(Duration::from_secs(5)));
}

#[test]
/// Test that multiple threads ARE FASTER than only 1 thread
fn multiple_threads_faster_than_one() {

    // Get the duration of one worker test
    let hercules = Hercules::new(1);

    let now = Instant::now();
    for i in 0..LABOUR_COUNT {
        hercules.push_labour(move || { println!("Executing labour #{}", i.clone()); });
    }

    let one_worker_duration = Instant::now() - now;
    

    // Get the duration of multiple worker test
    let hercules = Hercules::new(tools::get_logical_core_count());

    let now = Instant::now();
    for i in 0..LABOUR_COUNT {
        hercules.push_labour(move || { println!("Executing labour #{}", i.clone()); });
    }

    let multiple_worker_duration = Instant::now() - now;

    println!("{:?} <= {:?}", multiple_worker_duration, one_worker_duration);

    // Multiple worker duration should be smaller that 1 worker
    assert!(multiple_worker_duration <= one_worker_duration);

}




