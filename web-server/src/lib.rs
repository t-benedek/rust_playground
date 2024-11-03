#![allow(warnings)]
use std::thread::{self, JoinHandle};
use std::sync::{mpsc, Arc, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize, 
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    ) -> Worker {
        let thread = thread::spawn(move || loop {

            let message = receiver
            .lock()
            .unwrap()
            .recv();
            
            match (message) {
                Ok(job) =>  {
                    println!("Worker {id} gota job; executing");
                    job();        
                }

                Err(_) => {
                    // when recv() returns an error, we are shutting down as the channel for this thread was closed
                    println!("Worker {id} shuttung down ... ");
                    break;
                },
            }

            
        });
        Worker { 
            id, 
            thread: Some(thread),
        }
    } 
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // dropping sender closes the channel. This causes an error the receivers in 'recv()' call
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker with id {}", worker.id);
            
            // we have to use 'take()' to obtain ownership of the worker thread 
            // if we want to call 'join()' that takes ownership
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();    
            }
            
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads used in the pool
    /// 
    /// # Panics
    /// 
    /// The 'new' function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(
                Worker::new(id, Arc::clone(&receiver))
            );
        }
        ThreadPool { 
            workers,
            sender: Some(sender),
        }
    }

    /// Execute given closure
    /// 
    /// # Panics
    /// 
    /// The 'execute' function will panic if there is no channel with sender available or no job available to be executed
    pub fn execute<F>(&self, closure:F) 
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(closure);

        self.sender
            .as_ref()
            .unwrap()
            .send(job)
            .unwrap();
    }
}