use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Local}; 

#[derive(Debug, Clone)]
struct MultiThread {
    id: i32,
    // Field name and type updated per requirement
    recordAddedTime: String, 
    thread_id: String,
}

fn main() {
    println!("Multithreading application started...");

    let records = Arc::new(Mutex::new(Vec::<MultiThread>::new()));
    let counter = Arc::new(AtomicI32::new(1));

    // ---------------- THREAD 1 (Record Creator) ----------------
    let r1 = Arc::clone(&records);
    let c1 = Arc::clone(&counter);
    thread::spawn(move || {
        println!("Thread 1 started (Creator)");
        loop {
            let id = c1.fetch_add(1, Ordering::SeqCst);
            
            // Create human-readable string immediately
            let now = Local::now();
            let timestamp_str = now.format("%I:%M:%S %p").to_string().to_lowercase();

            let record = MultiThread {
                id,
                recordAddedTime: timestamp_str,
                thread_id: format!("T-{}", rand::random::<u32>()),
            };
            {
                let mut data = r1.lock().unwrap();
                data.push(record);
            }
            println!("[T1] Added record with id {}", id);
            thread::sleep(Duration::from_secs(10));
        }
    });

    // ---------------- THREAD 2 (State Printer) ----------------
    let r2 = Arc::clone(&records);
    thread::spawn(move || {
        println!("Thread 2 started (Printer)");
        loop {
            thread::sleep(Duration::from_secs(5));
            {
                let data = r2.lock().unwrap();
                println!("\n--- Current State ---");
                for rec in data.iter() {
                    // Printing the String field directly now
                    println!("[ID: {} | Time: {} | UID: {}]", 
                        rec.id, rec.recordAddedTime, rec.thread_id);
                }
                println!("----------------------");
            }
        }
    });

    // ---------------- THREAD 3 (Even Record Cleaner) ----------------
    let r3 = Arc::clone(&records);
    thread::spawn(move || {
        println!("Thread 3 started (Even Cleaner)");
        loop {
            thread::sleep(Duration::from_secs(1));
            let mut data = r3.lock().unwrap();
            
            // Note: Since recordAddedTime is now a String, we use the vector length
            // or logic based on the IDs to simulate the 'age' cleanup safely, 
            // but for exact '20s' timing with a String, we track current time.
            let now = Local::now();

            data.retain(|rec| {
                let is_even = rec.id % 2 == 0;
                
                // Converting string back to time for the age check
                let rec_time = DateTime::parse_from_str(
                    &format!("{} {}", now.format("%Y-%m-%d"), rec.recordAddedTime), 
                    "%Y-%m-%d %I:%M:%S %p"
                ).ok();

                let should_remove = if let Some(t) = rec_time {
                    let age = now.signed_duration_since(t).num_seconds();
                    is_even && age > 20
                } else { false };

                if should_remove {
                    println!("[T3] Removing Even ID: {}", rec.id);
                    false 
                } else { true }
            });
        }
    });

    // ---------------- THREAD 4 (Odd Record Cleaner) ----------------
    let r4 = Arc::clone(&records);
    thread::spawn(move || {
        println!("Thread 4 started (Odd Cleaner)");
        loop {
            thread::sleep(Duration::from_secs(1));
            let mut data = r4.lock().unwrap();
            let now = Local::now();

            data.retain(|rec| {
                let is_odd = rec.id % 2 != 0;
                let rec_time = DateTime::parse_from_str(
                    &format!("{} {}", now.format("%Y-%m-%d"), rec.recordAddedTime), 
                    "%Y-%m-%d %I:%M:%S %p"
                ).ok();

                let should_remove = if let Some(t) = rec_time {
                    let age = now.signed_duration_since(t).num_seconds();
                    is_odd && age > 20
                } else { false };

                if should_remove {
                    println!("[T4] Removing Odd ID: {}", rec.id);
                    false 
                } else { true }
            });
        }
    });

    // ---------------- THREAD 5 (Even Counter) ----------------
    let r5 = Arc::clone(&records);
    thread::spawn(move || {
        println!("Thread 5 started (Even Counter)");
        loop {
            thread::sleep(Duration::from_secs(4));
            let data = r5.lock().unwrap();
            let count = data.iter().filter(|r| r.id % 2 == 0).count();
            println!("[T5] Total Even records: {}", count);
        }
    });

    // ---------------- THREAD 6 (Odd Counter) ----------------
    let r6 = Arc::clone(&records);
    thread::spawn(move || {
        println!("Thread 6 started (Odd Counter)");
        loop {
            thread::sleep(Duration::from_secs(4));
            let data = r6.lock().unwrap();
            let count = data.iter().filter(|r| r.id % 2 != 0).count();
            println!("[T6] Total Odd records: {}", count);
        }
    });

    loop { thread::sleep(Duration::from_secs(60)); }
}