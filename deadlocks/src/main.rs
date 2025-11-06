use std::sync::Mutex;

static MY_SHARED: Mutex<u32> = Mutex::new(3);

fn poisoner() {
    let mut lock = MY_SHARED.lock().unwrap();
    *lock += 1;
    panic!("poinsoned")
}
fn main() {
    let handle = std::thread::spawn(poisoner);
    println!("trying to return from the thread ");
    print!("{:?}", handle.join());

    let lock = MY_SHARED.lock();
    println!("mutex is {:?}", lock);
}
