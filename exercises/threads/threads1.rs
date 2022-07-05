// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::{Arc,Mutex}; // add mutex.
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // soo... mutex.
    // mutex provides data integrity by causing a 'record' or a piece of data to be locked before
    // allowing someone else to use it. A lock can only occur as long as the call is owned (if its unalive it drops and unlocks, if it panics we're screwed)
    // a mutex is still owned by one thing, so we cant just say spin up loads of threads and use mutex directly, it only has one owner so itll move into the first thread
    // its acquired by. We also can't just borrow it because only one thing at a time can have the lock (they have the talking crab and you dont >:( )
    // so we can use Arc, like Rc, it allows multiple ownership of the pointer, which in this case is owning a smart pointer to a shared data.
    // so Arc -> Mutex -> Data makes it so multiple threads can access and modify one data but only as long as they have talking crab (and you dont)

    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut v = status_shared.lock().unwrap(); // Unwrap NOT RECCOMENDED!
            (*v).jobs_completed += 1;       // Should unlock after this... We used * to get direct access to the data (hidden behind pointer mutexguard)
        }
    });
    while (*status.lock().unwrap()).jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
