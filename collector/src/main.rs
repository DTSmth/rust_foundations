use std::io::Write;
use shared_data::{encode_v1, CollectorCommandV1, DATA_COLLECTOR_ADDRESS};
use sysinfo::System;
use std::{time::Instant, sync::mpsc::Sender};
use std::collections::VecDeque;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollectorError {
    #[error("Unable to connect")]
    UnableToConnect,
    #[error("Sending failed")]
    UnableToSendData,
}

pub fn collect_data(tx: Sender<CollectorCommandV1>) {
    // Initialize the sysinfo system
    let mut sys = sysinfo::System::new_all();

    // Perform a single refresh and pause. `sysinfo` gathers data via deltas,
    // and the first reading is usually useless.
    sys.refresh_memory();
    sys.refresh_cpu_all();
    std::thread::sleep(std::time::Duration::from_secs_f32(1.0));

    // Run forever
    loop {
        // Note when we're starting
        let now = Instant::now();

        // Refresh the stored data
        sys.refresh_memory();
        sys.refresh_cpu_all();

        // Get new values
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let num_cpus = sys.cpus().len();
        let total_cpu_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>();
        let average_cpu_usage = total_cpu_usage / num_cpus as f32;

        // Submit
        let send_result = tx.send(CollectorCommandV1::SubmitData {
            collector_id: 0,
            total_memory,
            used_memory,
            average_cpu_usage,
        });
        if let Err(e) = send_result {
            println!("Error sending data: {e:?}");
        }

        // Wait for the next cycle
        let elapsed_seconds = now.elapsed().as_secs_f32();
        if elapsed_seconds < 1.0 {
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0 - elapsed_seconds));
        } else {
            // Warning: we're running behind!
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
        }
    }
}

pub fn send_command(bytes: &[u8]) -> Result<(), CollectorError> {
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;
    stream
        .write_all(bytes)
        .map_err(|_| CollectorError::UnableToSendData)?;

    Ok(())
}

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<CollectorCommandV1>();

    // Start the collector thread
    let _collector_thread = std::thread::spawn(move || {
        collect_data(tx);
    });

    let mut send_queue = VecDeque::with_capacity(120);

    // Listen for commands to send
    while let Ok(command) = rx.recv() {
        let encode = encode_v1(&command);
        send_queue.push_back(encode);

        while let Some(command) = send_queue.pop_front() {
            if send_command(&command).is_err() {
                println!("Error");
                send_queue.push_front(command);
                break;
            }
        }
        // let _ = send_command(command);
    }
}

