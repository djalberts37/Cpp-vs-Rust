use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() 
{
    let mut shared_value = AtomicUsize::new(0);

    let thread1 = thread::spawn(move || 
    {
        for _ in 0..1000000 
        {
            shared_value.fetch_add(1, Ordering::SeqCst);
        }
    });

    let thread2 = thread::spawn(move || 
    {
        for _ in 0..1000000 
        {
            shared_value.fetch_add(1, Ordering::SeqCst);
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("Shared_value = {}", shared_value.load(Ordering::SeqCst));
}

// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() 
// {
//     let shared_value = Arc::new(Mutex::new(0));

//     let thread1 = 
//     {
//         let shared_value = shared_value.clone();
//         thread::spawn(move || 
//         {
//             for _ in 0..1000000 
//             {
//                 let mut value = shared_value.lock().unwrap();
//                 *value += 1;
//             }
//         })
//     };

//     let thread2 = 
//     {
//         let shared_value = shared_value.clone();
//         thread::spawn(move || 
//         {
//             for _ in 0..1000000 
//             {
//                 let mut value = shared_value.lock().unwrap();
//                 *value += 1;
//             }
//         })
//     };

//     thread1.join().unwrap();
//     thread2.join().unwrap();

//     let value = shared_value.lock().unwrap();
//     println!("Shared_value = {}", *value);
// }