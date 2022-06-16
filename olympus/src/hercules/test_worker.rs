/*
 * @file hercules/test_worker.rs
 *
 * @module olympus::hercules::worker
 *
 * @brief Contains unit tests for Worker struct.
 * 
 * @details
 * Contains unit tests for Worker struct.
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-06-16
 *
 * @version
 * 1.0 : 2022-06-16 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 */

use std::sync::{mpsc::{self, Sender}, Arc, Mutex};
use super::worker::{Worker, WorkerMessage};

// Count of labor used to test
static LABOUR_COUNT: usize = 4096;

// Loop count for test
static LOOP_COUNT: usize = 256;

// Number of workers for multiple workers test
static WORKER_COUNT: usize = 4096;

// Count of stress tests
static STRESS_COUNT: usize = 256;

#[test]
/// Create a worker with a receiver and send a terminate message.
fn create_worker_with_receiver() {
    
    // Create sender and worker
    let (sender, workers) = create_worker_vector(1);
    
    // Make sure to terminate so no error happens
    terminate_worker_with_sender(&sender, workers);
}


#[test]
/// Create a worker with a receiver and push labours
fn create_worker_with_receiver_push_labours() {

    // Create sender and worker
    let (sender, workers) = create_worker_vector(1);

    // Push labours with senders
    push_labour_with_sender(&sender);
    
    // Make sure to terminate so no error happens
    terminate_worker_with_sender(&sender, workers);
}


#[test]
/// Create multiple workers with a receiver and push labours
fn create_multiple_workers_with_receiver_push_labours() {

    // Create sender and worker
    let (sender, workers) = create_worker_vector(WORKER_COUNT);

    // Push labours with senders
    push_labour_with_sender(&sender);
    
    // Make sure to terminate so no error happens
    terminate_worker_with_sender(&sender, workers);
}


#[test]
#[ignore]
/// Stress test worker
fn stress_test_worker() {
    for i in 0..STRESS_COUNT {
        println!("Stress #{} of {}...", i+1, STRESS_COUNT);
        create_worker_with_receiver();
        create_worker_with_receiver_push_labours();
        create_multiple_workers_with_receiver_push_labours();
    }
}


/**
 * FUNCTIONS USED IN TESTS
 */

// Create a sender/receiver channel and vector of workers and return the pair
fn create_worker_vector<'a>(worker_count:usize) -> (Sender<WorkerMessage>, Vec<Worker>) {
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(worker_count);

    println!("Creating worker...");
    for id in 0..worker_count {
        if id % 8 == 0 {
            print!("{} ", id);
        }
        workers.push(Worker::new(id, Arc::clone(&receiver)));
    }
    println!("");

    (sender, workers)
}

// Make an addition loop of up to 'n'
fn calc_add_loop(n : usize){
    let mut _c:usize = 0;
    for i in 0..n {
        _c += i;
    }
}

// Push labours with sender
fn push_labour_with_sender(sender : &Sender<WorkerMessage>){
    // Send 'LABOUR_COUNT' labours
    for _i in 0..LABOUR_COUNT {

        sender.send(WorkerMessage::NewLabour(Box::new(|| {
            calc_add_loop(LOOP_COUNT);
        }))).unwrap();
    }
}

// Terminate workers
fn terminate_worker_with_sender(sender : &Sender<WorkerMessage>, mut workers: Vec<Worker>){

    println!("Terminating worker...");
    for i in 0..workers.len() {
        if i % 8 == 0 {
            print!("{} ", i);
        }
        sender.send(WorkerMessage::Terminate).unwrap();
    }
    println!("");

    println!("Joining worker...");
    for i in 0..workers.len() {
        if i % 8 == 0 {
            print!("{} ", i);
        }

        if let Some(thread) = workers[i].thread.take() {
            thread.join().unwrap();
        }
    }
    println!("");

}