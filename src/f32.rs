use crate::{calculate_speedup_percent, process_multi_thread, process_single_thread};
use num_format::{Locale, ToFormattedString};
use std::thread;
use std::time::Instant;

pub fn run() {
    let test_sizes = vec![1_000_000, 10_000_000, 100_000_000, 1_000_000_000];

    for &num_values in &test_sizes {
        println!(
            "Crunching {} numbers...",
            num_values.to_formatted_string(&Locale::en)
        );

        let mut data_single: Vec<f32> = (0..num_values).map(|x| x as f32).collect();
        let mut data_multi: Vec<f32> = data_single.clone();

        let single_thread = thread::spawn(move || {
            let start = Instant::now();
            process_single_thread(&mut data_single);
            let single_duration = start.elapsed();
            single_duration
        });

        let multi_thread = thread::spawn(move || {
            let start = Instant::now();
            process_multi_thread(&mut data_multi);
            let multi_duration = start.elapsed();
            multi_duration
        });

        let single_duration = single_thread.join().unwrap();
        let multi_duration = multi_thread.join().unwrap();

        println!("Single-threaded time: {:?}", single_duration);
        println!("Multi-threaded time: {:?}", multi_duration);

        calculate_speedup_percent(single_duration, multi_duration);
        println!("------------------------------------------------------------");
    }
}
