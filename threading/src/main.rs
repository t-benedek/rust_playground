use std::thread;
use std::time::Duration;

fn main() {
    _thread_join();
    _thread_messages();
    thread_shared_state();
}

fn _thread_join() {
    let handle = thread::spawn(|| 
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        });
    
        // block the main thread until the spawned thread ended
        // handle.join().unwrap();
    
        for i in 1..5 {
            println!("Hi number {i} from the main thread");
            thread::sleep(Duration::from_millis(1));            
        };
    
        // putting the join here, both threads will run in parallel
        handle.join().unwrap();
}

fn _thread_messages() {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"), 
        ]; 
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        
            // this won't work because of ownership being transferred to the receiver
            // println!("{val}");
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
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    for received in rx {
        println!("Got: {received}");
    }
}

fn thread_shared_state() {
    use std::sync::{Arc, Mutex};

    // we can use Arc instead of Rc that can be used in multi-threading context for reference count
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
