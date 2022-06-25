/*
 * @file hercules/test/
 * taskmaster_test.rs
 *
 * @module olympus::hercules::test
 *
 * @brief Contains unit tests for Taskmaster struct.
 * 
 * @details
 * Contains unit tests for Taskmaster struct such as :
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

use rand::Rng;

use crate::hercules::{Taskmaster, TaskmasterNewOptions, WorkOrder};
use crate::tools;

use std::thread;
// To calculate difference in some test(s)
use std::time::{Duration, Instant};

// Count of labor used to test
static LABOUR_COUNT: usize = 65535;

// Loop size when calc
static LOOP_SIZE: usize = 1024;

// Stress test duration in seconds (5 mins)
static STRESS_DURATION: Duration = Duration::from_secs(300); 


#[test]
#[should_panic]
/// Test that Taskmaster CAN'T be created without workers (size == 0)
fn create_taskmaster_no_worker() {

    let _tsm = Taskmaster::new(TaskmasterNewOptions::SetWorkerCount(0));

}

#[test]
#[should_panic]
/// Test that Taskmaster CAN'T be created with more than tools::get_logical_core_count() workers.
fn create_taskmaster_too_many_worker() {

    let _tsm = Taskmaster::new(TaskmasterNewOptions::SetWorkerCount(tools::get_logical_core_count()+1));

}

#[test]
/// Creating with only 1 thread and pushing LABOUR_COUNT labour
fn create_taskmaster_with_one_worker() {

    let tsm = Taskmaster::new(TaskmasterNewOptions::MaximumWorkers);

    for _i in 0..LABOUR_COUNT {
        tsm.push_labour(move || { calc_add_loop(LOOP_SIZE); });
    }
}

#[test]
/// Creating with logical core count and pushing LABOUR_COUNT labour
fn create_taskmaster_with_logical_core_count_worker() {

    let tsm = Taskmaster::new(TaskmasterNewOptions::MaximumWorkers);

    for _i in 0..LABOUR_COUNT {
        tsm.push_labour(move || { calc_add_loop(LOOP_SIZE); });
    }
}

#[test]
/// Test that multiple threads ARE FASTER than only 1 thread.
fn multiple_threads_faster_than_one() {

   
    let one_worker_duration = get_work_order_runtime_duration(1);
    let multiple_worker_duration = get_work_order_runtime_duration(tools::get_logical_core_count());

    println!("{:?} <= {:?}", multiple_worker_duration, one_worker_duration);

    // Multiple worker duration should be smaller that 1 worker
    assert!(multiple_worker_duration <= one_worker_duration);

}

#[test]
#[ignore]
// Stress test Taskmaster
fn taskmaster_stress_test() {

    let core_count = tools::get_logical_core_count();
    let mut rng = rand::thread_rng();
    let mut stress_loop = 0;
    let started = Instant::now();

    while Instant::now() - started <= STRESS_DURATION {    
        stress_loop += 1;
        

        // Get a random number of workers from 1 to logical core count
        let wsize = rng.gen_range(1..core_count + 1);
        let batch_size = rng.gen_range(0..LABOUR_COUNT);
        let loop_size = rng.gen_range(0..LOOP_SIZE);

        println!("Stress #{} | Workers={}/{} | Batch={}/{} | Loop={}/{} | Remaining time : {:?}...", stress_loop, wsize, core_count,
            batch_size, LABOUR_COUNT, loop_size, LOOP_SIZE, if  Instant::now() - started < STRESS_DURATION {
                STRESS_DURATION - (Instant::now() - started)
            } else {
                Duration::from_secs(0)
            });

        {
            // Create Taskmaster with Core count
            let tsm = Taskmaster::new(TaskmasterNewOptions::SetWorkerCount(wsize));

            // Push 0 to LABOUR_COUNT jobs
            for _i in 0..batch_size {
                tsm.push_labour(move || { calc_add_loop(loop_size); });
            }
        }    
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

fn wait_function(){
    thread::sleep(Duration::from_millis(1));
}

// Get the duration of a work order execution time
fn get_work_order_runtime_duration(worker_count:usize) -> Duration {

    // Init Taskmaster and Work Order
    let tsm = Taskmaster::new(TaskmasterNewOptions::SetWorkerCount(worker_count));
    let wo = WorkOrder::new(Some(&tsm));

    println!("Running test with {} worker(s)", worker_count);

    // Get starting time
    let start_time = Instant::now();
    for _ in 0..5000 {
        wo.add_labour(move || { wait_function(); });
    }

    println!("Test with {} worker(s) finished pushing tasks in {:?}", worker_count, Instant::now() - start_time);

    wo.wait(None);

    let diff = Instant::now() - start_time;

    println!("Test with {} worker(s) finished in {:?}.", worker_count, diff);

    diff
}
