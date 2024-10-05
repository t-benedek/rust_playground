use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // thread_join();

    thread_messages();
}

fn thread_join() {
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

fn thread_messages() {
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
