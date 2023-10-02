use std::thread;
use std::time::{Instant};
use rayon::prelude::*;
use num_format::{Locale, ToFormattedString};

fn main() {
    let test_sizes = vec![1_000_000, 10_000_000, 100_000_000, 1_000_000_000];

    for &num_values in &test_sizes {
        println!("Crunching {} numbers...", num_values.to_formatted_string(&Locale::en));

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

fn process_single_thread(data: &mut [f32]) {
    for x in data.iter_mut() {
        *x *= 2.0;
    }
}

fn process_multi_thread(data: &mut [f32]) {
    data.par_chunks_mut(1024).for_each(|chunk| {
        for x in chunk {
            *x *= 2.0;
        }
    });
}

fn calculate_speedup_percent(single_duration: std::time::Duration, multi_duration: std::time::Duration) {
    let single_time = single_duration.as_secs_f64();
    let multi_time = multi_duration.as_secs_f64();

    if single_time > multi_time {
        let percent = ((single_time - multi_time) / single_time) * 100.0;
        println!("Multi-threaded is {:.2}% faster than single-threaded.", percent);
    } else {
        let percent = ((multi_time - single_time) / multi_time) * 100.0;
        println!("Single-threaded is {:.2}% faster than multi-threaded.", percent);
    };
}