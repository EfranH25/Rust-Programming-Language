use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

// Race condition Note:
// Race conditions, in which threads are accessing data or resources in an inconsistent order
// Deadlocks, in which two threads are waiting for each other, preventing both threads from continuing
// Bugs that only happen in certain situations and are hard to reproduce and fix reliably

fn example_thread() {
    // A channel has two halves: a transmitter and a receiver. The transmitter half is the upstream location where you put the rubber duck into the river, and the receiver half is where the rubber duck ends up downstream. One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages. A channel is said to be closed if either the transmitter or receiver half is dropped.

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1))
    }

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || println!("here's a vector: {v:?}"));
    // Wait for thread to finish. Blocks main thread from finishing. If main thread finishes, all other threads are terminated

    handle.join().unwrap();
}

fn example_mpsc() {
    // Creates transmitter and receiver for thread
    let (tx, rx) = mpsc::channel();

    // Use closer + move to move ownership in thread
    thread::spawn(move || {
        // let val = String::from("hi");
        // tx.send(val).unwrap();
        // println!("val is {val}");
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let received = rx.recv().unwrap();
    // println!("Got: {received}");
    for received in rx {
        println!("Got: {received}")
    }
}

fn example_mpsc_multi() {
    let (tx, rx) = mpsc::channel();

    // Clones transmitter, making a new transmitter > new thread
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

fn example_mutexes() {
    // Mutex is an abbreviation for mutual exclusion, as in a mutex allows only one thread to access some data at any given time. To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock. The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data. Therefore, the mutex is described as guarding the data it holds via the locking system.
    // Mutexes have a reputation for being difficult to use because you have to remember two rules:
    //  You must attempt to acquire the lock before using the data.
    //  When you’re done with the data that the mutex guards, you must unlock the data so that other threads can acquire the lock.
    // Anology:
    //  For a real-world metaphor for a mutex, imagine a panel discussion at a conference with only one microphone. Before a panelist can speak, they have to ask or signal that they want to use the microphone. When they get the microphone, they can talk for as long as they want to and then hand the microphone to the next panelist who requests to speak. If a panelist forgets to hand the microphone off when they’re finished with it, no one else is able to speak. If management of the shared microphone goes wrong, the panel won’t work as planned!

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Spawned thread count {num}");
            thread::sleep(Duration::from_secs(1));
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
fn main() {
    // Note: Very little of how Rust handles concurrency is part of the language, many concurrency solutions are implemented as crates. These evolve more quickly than the standard library, so be sure to search online for the current, state-of-the-art crates to use in multithreaded situations.

    // example_thread()
    // example_mpsc()
    // example_mpsc_multi()
    example_mutexes()
}
