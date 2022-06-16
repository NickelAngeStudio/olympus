/*
 * @file hercules/test_work_order.rs
 *
 * @module olympus::hercules
 *
 * @brief Contains unit tests for Work Order struct.
 * 
 * @details
 * Contains unit tests for Work Order struct.
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
 */
use crate::Hercules;
use crate::WorkOrder;
use crate::tools;
use std::time::{Duration};
use std::{thread};
use super::work_order::WorkOrderWaitResult;

// Count of labor used to test
static LABOUR_COUNT: usize = 65536;

// How many Hercules we create
static HERCULES_COUNT: usize = 16;

// How many time we add and wait
static ADD_AND_WAIT_COUNT: usize = 16;

// How many work order we create
static WORK_ORDER_COUNT: usize = 64;

// Stress test parameters
static STRESS_COUNT: usize = 256;
static STRESS_HERCULES_COUNT: usize = 4;
static STRESS_WO_COUNT: usize = 8;
static STRESS_LABOUR_COUNT: usize = 4096;
static STRESS_LOOP_N: usize = 4096;


#[test]
#[should_panic]
/// Test that a WorkOrder can't be created without a valid Hercules reference.
fn create_work_order_no_hercules() {

    let _wo = WorkOrder::new(None);
}


#[test]
/// Create a Work Order for Hercules then add labours to it.
fn create_work_order_then_wait() {

    let hercules = Hercules::new(tools::get_logical_core_count());
    let wo = WorkOrder::new(Some(&hercules));

    // Add fn to work order
    add_fn_to_work_order(&wo, LABOUR_COUNT, 500);

    // Wait a max of 5 secs. Panic if more than 5 secs.
    work_order_wait_or_panic(&wo, Some(Duration::from_secs(5)));    
}


#[test]
#[should_panic]
/// Create a Work Order that WILL Timeout.
fn create_work_order_with_timeout() {

    let hercules = Hercules::new(tools::get_logical_core_count());
    let wo = WorkOrder::new(Some(&hercules));

    // Block all threads for 2 secs
    for _ in 0..tools::get_logical_core_count() {
        wo.add_labour(move || { 
            thread::sleep(Duration::from_secs(2));
        });
    }

    // Wait a max of 1 secs. Panic if more than 1 secs.
    work_order_wait_or_panic(&wo, Some(Duration::from_secs(1)));    
}


#[test]
/// Adding labours after waiting, repeat 'ADD_AND_WAIT_COUNT' times
fn add_labour_after_waiting() {
    let hercules = Hercules::new(tools::get_logical_core_count());
    let wo = WorkOrder::new(Some(&hercules));

    // Add labours and wait 'ADD_AND_WAIT_COUNT' times
    for i in 0..ADD_AND_WAIT_COUNT {
        println!("Loop #{} : Adding {} fn...", i+1, LABOUR_COUNT);
        add_fn_to_work_order(&wo, LABOUR_COUNT, 500);

        println!("Loop #{} : Wait 5 secs max...", i+1);
        work_order_wait_or_panic(&wo, Some(Duration::from_secs(5)));    
    }
}


#[test]
/// Multiple work orders, up to 'WORK_ORDER_COUNT'
fn handle_multiple_work_orders() {

    test_multiple_hercules_and_multiple_work_order(1, WORK_ORDER_COUNT, LABOUR_COUNT,
         500, Some(Duration::from_secs(5)), true);
    
}


#[test]
#[ignore]
/// Multiple work orders and hercules. (long duration)
fn handle_multiple_work_orders_and_hercules() {

    test_multiple_hercules_and_multiple_work_order(HERCULES_COUNT, WORK_ORDER_COUNT, LABOUR_COUNT,
        500, Some(Duration::from_secs(5)), true);

}


#[test]
#[ignore]
/// Stress test. (really long duration)
fn work_order_stress_test() {
    // Test multiple time to try to trigger memory leaks or crashes
    for i in 0..STRESS_COUNT {
        println!("Stress #{} of {}...", i+1, STRESS_COUNT);
        test_multiple_hercules_and_multiple_work_order(STRESS_HERCULES_COUNT, STRESS_WO_COUNT, STRESS_LABOUR_COUNT,
            STRESS_LOOP_N, Some(Duration::from_secs(5)), false);
    }

}

/**
 * FUNCTIONS USED IN TESTS
 */
// Make an addition loop of up to 'n'
fn calc_add_loop(n : usize){
    let mut _c:usize = 0;
    for i in 0..n {
        _c += i;
    }
}

// Add 'fn_count' labour of size 'loop_n' in the work order
pub fn add_fn_to_work_order(wo:&WorkOrder, fn_count:usize, loop_n : usize){
    for _i in 0..fn_count {
        wo.add_labour(move || { calc_add_loop(loop_n); });
    }
}

// Work order will wait and/or panic
pub fn work_order_wait_or_panic(wo:&WorkOrder, timeout: Option<Duration>) -> WorkOrderWaitResult{

    match wo.wait(timeout) {
        super::work_order::WorkOrderWaitResult::Done => {

        },
        super::work_order::WorkOrderWaitResult::Timeout => {
            panic!("work_order_wait_or_panic() timed out!");
        },
    }

    // Return that wait is done
    WorkOrderWaitResult::Done 
}

// Fn that test 1..n Hercules with 1..n work orders adding fn_count fn of loop_n size with timeout option
fn test_multiple_hercules_and_multiple_work_order(hercules_count: usize, work_order_count: usize, fn_count:usize, loop_n:usize, timeout: Option<Duration>, verbose: bool){

    let mut h_vec: Vec<Hercules> = Vec::with_capacity(hercules_count);
    let mut wo_vec: Vec<WorkOrder> = Vec::with_capacity(work_order_count * hercules_count);

    // Create Hercules
    for i in 0..hercules_count {
        if verbose {
            println!("Hercules#{} : creating...", i+1);
        }
        h_vec.push(Hercules::new(tools::get_logical_core_count()));
    }

    // Create Hercules Work Orders
    for i in 0..hercules_count {
        for j in 0..work_order_count {
            if verbose {
                println!("Hercules#{} : WO#{} creating...", i+1, j+1);
            }
            wo_vec.push(WorkOrder::new(Some(&h_vec[i])));
        }
    }

    // Add labours in all WO
    for i in 0..work_order_count * hercules_count {
        if verbose {
            println!("WO#{} : adding labours...", i+1);
        }
        add_fn_to_work_order(&wo_vec[i], fn_count, loop_n);
    }

    // Wait all WO
    for i in 0..work_order_count * hercules_count {
        if verbose {
            println!("WO#{} : waiting...", i+1);
        }
        work_order_wait_or_panic(&wo_vec[i], timeout); 
    }

}