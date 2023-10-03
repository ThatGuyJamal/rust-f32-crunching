use std::thread;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use crate::{calculate_speedup_percent, process_multi_thread, process_single_thread};

type FloatVector = Arc<Mutex<Vec<f32>>>;

pub fn run() {
    let vector_sizes: Vec<i32> = vec![1_000_000, 10_000_000, 100_000_000, 1_000_000_000];
    let chunk_size: usize = 4; // Define the number of vectors to process in each chunk
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

        let single_threads: Vec<JoinHandle<Duration>> = chunk.iter().map(|data| {
            let data: Arc<Mutex<Vec<f32>>> = data.clone();
            thread::spawn(move || {
                let start = Instant::now();
                process_single_thread(&mut data.lock().unwrap());
                let single_duration = start.elapsed();
                single_duration
            })
        }).collect();

        let multi_threads: Vec<JoinHandle<Duration>> = chunk.iter().map(|data| {
            let data: Arc<Mutex<Vec<f32>>> = data.clone();
            thread::spawn(move || {
                let start: Instant = Instant::now();
                process_multi_thread(&mut data.lock().unwrap());
                let multi_duration: Duration = start.elapsed();
                multi_duration
            })
        }).collect();

        let single_durations: Vec<Duration> = single_threads.into_iter().map(|t: JoinHandle<Duration>| t.join().unwrap()).collect();
        let multi_durations: Vec<Duration> = multi_threads.into_iter().map(|t: JoinHandle<Duration>| t.join().unwrap()).collect();

        for (i, (single_duration, multi_duration)) in single_durations.iter().zip(multi_durations.iter()).enumerate() {
            let vector_num: usize = chunk_start + i + 1;
            println!("Vector {} size {} Single-threaded time: {:?}", vector_num, vector_sizes[vector_num - 1], single_duration);
            println!("Vector {} size {} Multi-threaded time: {:?}", vector_num, vector_sizes[vector_num - 1], multi_duration);
            calculate_speedup_percent(*single_duration, *multi_duration);
            println!("------------------------------------------------------------");
        }

        chunk_start = chunk_end;
    }
}
