use crate::{calculate_speedup_percent, process_multi_thread, process_single_thread};
use num_format::{Locale, ToFormattedString};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

type FloatVector = Arc<Mutex<Vec<f32>>>;

pub fn run() {
    let vector_sizes: Vec<i32> = vec![1_000_000, 10_000_000, 100_000_000, 1_000_000_000];
    let chunk_size: usize = 10_000; // Define the number of vectors to process in each chunk
    let mut data_vectors: Vec<FloatVector> = Vec::new();

    // Create and populate the vectors with different sizes
    for &size in &vector_sizes {
        let data: FloatVector = Arc::new(Mutex::new((0..size).map(|x| x as f32).collect()));
        data_vectors.push(data.clone());
    }

    let mut chunk_start: usize = 0;

    while let Some(chunk_end) = chunk_start.checked_add(chunk_size) {
        let chunk_end: usize = chunk_end.min(data_vectors.len());

        let chunk: Vec<_> = data_vectors[chunk_start..chunk_end].to_vec();

        let chunk_clone = chunk.clone();

        let single_thread: JoinHandle<Vec<Duration>> = thread::spawn(move || {
            let d: Vec<Duration> = chunk_clone
                .iter()
                .map(|data| {
                    let data: Arc<Mutex<Vec<f32>>> = data.clone();
                    let start = Instant::now();
                    process_single_thread(&mut data.lock().unwrap());
                    let single_duration = start.elapsed();
                    single_duration
                })
                .collect();
            d
        });

        let multi_thread: JoinHandle<Vec<Duration>> = thread::spawn(move || {
            let d: Vec<Duration> = chunk
                .iter()
                .map(|data| {
                    let data: Arc<Mutex<Vec<f32>>> = data.clone();
                    let start = Instant::now();
                    process_multi_thread(&mut data.lock().unwrap());
                    let multi_duration = start.elapsed();
                    multi_duration
                })
                .collect();
            d
        });

        let single_durations: Vec<Duration> = single_thread.join().unwrap();
        let multi_durations: Vec<Duration> = multi_thread.join().unwrap();

        for (i, (single_duration, multi_duration)) in single_durations
            .iter()
            .zip(multi_durations.iter())
            .enumerate()
        {
            let vector_num: usize = chunk_start + i + 1;
            let formatted_size = vector_sizes[vector_num - 1].to_formatted_string(&Locale::en);

            println!(
                "Vector {} size {} Single-threaded time: {:?}",
                vector_num,
                formatted_size.clone(),
                single_duration
            );
            println!(
                "Vector {} size {} Multi-threaded time: {:?}",
                vector_num, formatted_size, multi_duration
            );
            calculate_speedup_percent(*single_duration, *multi_duration);
            println!("------------------------------------------------------------");
        }

        chunk_start = chunk_end;
    }
}
