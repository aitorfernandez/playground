use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn main() {
    // single_thread_data and single_thread_data_clone shared ownership of value 5
    // in a single-thread context
    let single_thread_data = Rc::new(5);
    let single_thread_data_clone = Rc::clone(&single_thread_data);

    println!("single-thread data: {single_thread_data_clone}");

    // Rc<T> with RefCell<T> allows mutable access while having sharing ownership
    // shared ownership of the RefCell and allow mutable using RefCell
    let single_thread_shared_data = Rc::new(RefCell::new(5));
    let single_thread_shared_data_clone = Rc::clone(&single_thread_shared_data);

    *single_thread_shared_data.borrow_mut() += 1;
    *single_thread_shared_data_clone.borrow_mut() += 1;

    println!(
        "single-thread-shared data: {}",
        single_thread_shared_data_clone.borrow() // 7
    );

    // Arc<T>
    let across_thread_data = Arc::new(5);
    let across_thread_data_clone = Arc::clone(&across_thread_data);

    let handle = thread::spawn(move || println!("multi-thread: {across_thread_data_clone}"));
    handle.join().unwrap();

    // Exclusive access
    let shared_data = Arc::new(Mutex::new(5));
    let mut handles = vec![];

    for _ in 0..3 {
        let data_clone = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            let mut num = data_clone.lock().unwrap();
            *num += 1;
            println!("modified by thread: {}", num);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!(
        "shared-data exclusive access value: {}",
        shared_data.lock().unwrap()
    );

    // Shared
    let data = Arc::new(RwLock::new(5));
    let mut handles = vec![];

    for _ in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let num = data_clone.read().unwrap();
            println!("read-only access: {}", num);
        });
        handles.push(handle);
    }

    let data_clone = Arc::clone(&data);
    let writer_handle = thread::spawn(move || {
        let mut num = data_clone.write().unwrap();
        *num += 10;
        println!("write access: {}", num);
    });
    handles.push(writer_handle);

    for h in handles {
        h.join().unwrap();
    }

    println!("final value: {}", data.read().unwrap());
}
