use std::{sync::{Arc, Mutex, mpsc::{self, Receiver, Sender}}, thread::{self, JoinHandle}};


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);

        let (sender, reciever) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(reciever));

        let mut workers: Vec<Worker> = Vec::with_capacity(capacity);
        for id in 0..capacity {
            workers.push(Worker::new(id as u8, Arc::clone(&receiver)));
        }  
        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}

struct Worker {
    id: u8,
    thread: JoinHandle<()>
}

impl Worker {
    
    fn new(id: u8, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || { 
            loop {
                let Ok(job) = receiver.lock().unwrap().recv() else {
                    println!("Worker {id} shutting down...");
                    break;
                };

                println!("Worker {id} got a job; executing.");

                job();
            }});
        Worker { id, thread }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

