use std::fmt::Display;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
pub struct PoolCreationError {}
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Errors
    ///
    /// The `new` function will return `PoolCreationError` error type if the size is zero.
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError {});
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, closure: F) 
    where
        F: FnOnce() + Send + 'static,
    {
        let f = Box::new(closure);

        self.sender.send(Message::NewJob(f)).unwrap();
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker 
    {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                },
                Message::Terminate => {
                    println!("Worker {} was told to terminate; terminating.", id);
                    break;
                },
            }
        });

        Worker { id, thread: Some(thread) }
    }
}

impl Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Number of threads must be greater than 0")
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}.", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}