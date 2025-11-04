use std::thread;

fn my_thread() {
    println!("Hello, thread! {}", std::thread::current().name().unwrap());
}

fn main() {
    let handle = thread::Builder::new()
        .name("named".to_string())
        .stack_size(std::mem::size_of::<usize>() * 4)
        .spawn(my_thread)
        .unwrap();
    handle.join().unwrap();
}
