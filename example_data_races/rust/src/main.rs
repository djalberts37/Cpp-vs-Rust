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
