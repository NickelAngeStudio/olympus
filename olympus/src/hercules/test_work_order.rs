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
 * Test waiting
 */

use crate::Hercules;
use crate::WorkOrder;

use crate::tools;

// To calculate difference in some test(s)
use std::time::{Duration};

// Count of labor used to test
static LABOUR_COUNT: u32 = 65536;

// Quick function to be executed
fn quick_calc_fn() {

    let mut _c:u32 = 0;
    for i in 0..500 {
        _c += i;
    }

}

// Long function to be executed
fn long_calc_fn() {

    let mut _c:u32 = 0;
    for i in 0..65536 {
        _c += i;
    }

}


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

    for _i in 0..LABOUR_COUNT {
        wo.add_labour(move || { quick_calc_fn(); });
    }

    // Wait a max of 5 secs. Panic if more than 5 secs.
    match wo.wait(Some(Duration::from_secs(5))) {
        super::work_order::WorkOrderWaitResult::Done => {

        },
        super::work_order::WorkOrderWaitResult::Timeout => {
            panic!("create_work_order_then_wait() timedout! (it shouldn't)");
        },
    }
    
}

#[test]
#[should_panic]
/// Create a Work Order that WILL Timeout.
fn create_work_order_with_timeout() {

    let hercules = Hercules::new(tools::get_logical_core_count());
    let wo = WorkOrder::new(Some(&hercules));

    for _i in 0..LABOUR_COUNT {
        wo.add_labour(move || { quick_calc_fn(); });
        //hercules.push_labour(move || { calc_fn(); });
    }

    // Waiting
    
    println!("Todo before={}", wo.get_labour_remaining());

    println!("Waiting");
    wo.wait(Some(Duration::from_secs(5)));
    
    println!("Todo after={}", wo.get_labour_remaining());
    //hercules.wait(Some(Duration::from_secs(5)));

}

#[test]
/// TODO : Adding labours after waiting
fn add_labour_after_waiting() {
}

#[test]
/// TODO : Multiple work orders
fn handle_multiple_work_orders() {
}

#[test]
/// TODO : Multiple work orders and hercules
fn handle_multiple_work_orders_and_hercules() {
}

#[test]
#[ignore]
/// TODO : Stress test
fn work_order_stress_test() {
}