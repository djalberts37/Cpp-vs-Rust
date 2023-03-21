fn main() 
{
    let mut ptr = Box::new(42);

    drop(ptr);

    let value = *ptr;
    println!("Value = {}", value);
}

