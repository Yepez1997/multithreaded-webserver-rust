# multithreaded-webserver-rust
multi-threaded-webserver built with rust. project from rust docs. 


```rust
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // let thread = thread::spwan(|| {receiver} );
        let thread = thread::spawn(move || loop {
            // lock to aquire the mutex
            // if the lock is successfully receievd, call recv to receive a job from the channel 
            // recv blocks if there is no job 
            // 
            let job = receiver.lock().unwrap().recv().unwrap();
            println("Worker {} received a job and executing", id);
            job();
        })
        Worker {id, thread};
    }
}
```

```rust
impl ThreadPool {
    // create a new ThreadPool 

    /// new function will panic if the thread pool size is 0. 
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError>{
        assert!(size > 0);


        // allow thread pool to communicate with the worker nodes 
        let (sender, receiver) = mpsc::channel(); // channel to comunicate between sender + receiver. 
        // create threads and store them in the vector pool.
        // pre allocate the space in the vector.

        // thread safe reference counting pointer: https://doc.rust-lang.org/std/sync/struct.Arc.html
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

    
        // number of threads
        for i in 0..size {
            workers.push(Worker::new(i), Arc::clone(&receiver));
        }

        ThreadPool {workers, sender}

    }


    // execute spawns a thread
    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static, {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
```
