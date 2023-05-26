use std::thread;
use std::sync::{mpsc, Arc, Mutex};

type WorkerJob = Box<dyn FnBox + Send + 'static>;

enum WorkerMessage {
    NewJob(WorkerJob),
    Terminate,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<WorkerMessage>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// Panics if size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// Send function to be executed to a worker thread
    /// 
    /// # Panics
    /// 
    /// Panics if job can not be send.
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(WorkerMessage::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(WorkerMessage::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

/// Thread Pool Worker that handles a single thread
struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Spawn new worker that waits for a messages to be executed from the receiver
    /// 
    /// # Panics
    /// 
    /// Panics if message can not be received.
    fn new(receiver: Arc<Mutex<mpsc::Receiver<WorkerMessage>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    WorkerMessage::NewJob(job) => job.call_box(),
                    WorkerMessage::Terminate => break,
                };
            }
        });

        Worker { thread: Some(thread) }
    }
}
