use std::sync::atomic::{AtomicU64, AtomicUsize};
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;

fn main() {
    let mut non_static_value = 42;
    std::thread::scope(|s| {
        for i in 0..10 {
            s.spawn(|| {
                for i in 0..10 {
                    non_static_value += i;
                    println!("{}", non_static_value);
                    std::thread::sleep(Duration::from_millis(10));
                }
            });
        }
    });
}