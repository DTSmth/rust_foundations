use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;

// static mut COUNTER: i32 = 0;
static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    let mut handles = Vec::new();
    for _ in 0..1000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..100 {
                COUNTER.fetch_add(1, Relaxed);
            }
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("{}", COUNTER.load(Relaxed));
}
