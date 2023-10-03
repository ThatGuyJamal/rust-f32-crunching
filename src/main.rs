use rayon::prelude::*;
use std::env;
use std::process::exit;

mod f32;
mod f32_vec;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a test type: normal, vec, or all");
        return;
    }

    let test_type = &args[1];

    match test_type.as_str() {
        "normal" => {
            f32::run();
            exit(0)
        },
        "vec" => {
            f32_vec::run();
            exit(0)
        },
        "all" => {
            f32::run();
            f32_vec::run();
            exit(0)
        },
        _ => println!("Please provide a test type: normal or vec")
    }
}

pub fn process_single_thread(data: &mut [f32]) {
    for x in data.iter_mut() {
        *x *= 2.0;
    }
}

pub fn process_multi_thread(data: &mut [f32]) {
    data.par_chunks_mut(1024).for_each(|chunk| {
        for x in chunk {
            *x *= 2.0;
        }
    });
}

pub fn calculate_speedup_percent(single_duration: std::time::Duration, multi_duration: std::time::Duration) {
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