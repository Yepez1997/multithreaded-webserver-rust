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
