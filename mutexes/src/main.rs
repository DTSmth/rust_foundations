use std::sync::Mutex;

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = std::thread::spawn(move || {
            let mut lock = NUMBERS.lock().unwrap();
            lock.push(i);
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    let lock = NUMBERS.lock().unwrap();
    println!("{:#?}", lock)
}
