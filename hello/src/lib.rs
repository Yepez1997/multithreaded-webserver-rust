// holds thread pool implementation 

use std::thread;
use std::sync::mpsc;

// preventing race conditions and limiting a job to single worker 
use std::sync::Arc;
use std::sync::Mutex;

type Job = Box(dyn, FnOnce()) + Send + 'static>;

// used to signal to exit a thread 
enum Message {
    NewJob(Job);
    Terminate,
}

// channel to communicate between different threads 

// create a channel and hold on the receiving side. 
pub struct ThreadPool {

    // each of the threads returns a Join handle method 
    // refer to thread::spawn documentation.

    // threads: Vec<thread::JoinedHandle<()>>,
    workers: Vec<Worker>
    // send jobs to workers
    sender: mpsc::Sender<Message>
}


// executes connections asychronously

// worker will create threads and have them wait 
// sends code from thread pool to a thread.

//  side note: similar to go interface
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

        // pool of workers that are listenting to receive a job, and a sender 
        ThreadPool {workers, sender}

    }


    // execute spawns a thread
    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static, {
        let job = Box::new(f);
        // send the job to a worker 
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// should join all threads and make sure the threads finish their work 
// i.e if something bad happens, the worker should be able to 
// finish their proccess
// stop accepting requests
impl Drop for ThreadPool {
    fn drop(&mut self) {

        // terminate workers i.e do not allow any more requests
        for worker in self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // allow workers to finish
        for worker in self.workers {
            println!("Shutting down worker {}", worker.id);
            // worker.thread.join().unwrap()
            // pull thread out of the worker 
            // recall the take method takes some variant out and leaves none in place
            if let Some(thread) =  worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// workers hold the receiving side of the channell
struct Worker {
    id: usize,
    thread: Option<thread.JoinedHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // let thread = thread::spwan(|| {receiver} );
        let thread = thread::spawn(move || loop {
            // lock to aquire the mutex
            // if the lock is successfully receievd, call recv to receive a job from the channel 
            // recv blocks if there is no job 
            // unwrap is temp.

            let message = receiver.lock().unwrap().recv().unwrap();
            
            // similair to go channels ...
            match message { 
                Message::NewJob(job) => {
                    println("Worker {} received a job and executing", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }

        })
        Worker {
            id, 
            Some(thread)
        };
    }
}

