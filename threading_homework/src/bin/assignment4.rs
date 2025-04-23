use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;

    // Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    // Create 2 producer threads
    let mut producer_handles = vec![];
    for id in 1..=2 {
        let tx_clone = tx.clone();
        producer_handles.push(thread::spawn(move || {
            producer(id, tx_clone, ITEM_COUNT / 2);
        }));
    }

    // Create 3 consumer threads
    let mut consumer_handles = vec![];
    for id in 1..=3 {
        let rx_clone = Arc::clone(&rx);
        consumer_handles.push(thread::spawn(move || {
            consumer(id, rx_clone);
        }));
    }

    // Wait for all producer threads to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }

    // Send termination signals to consumers
    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for all consumer threads to finish
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..item_count {
        let value = rng.gen_range(1..=100);
        println!("Producer {} generated value: {}", id, value);
        tx.send(value).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("Producer {} finished producing.", id);
}

// Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let value = {
            let rx = rx.lock().unwrap();
            rx.recv().unwrap()
        };

        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal.", id);
            break;
        }

        println!("Consumer {} processed value: {}", id, value);
        thread::sleep(Duration::from_millis(150));
    }
}