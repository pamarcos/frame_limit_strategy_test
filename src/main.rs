extern crate num_cpus;

#[macro_use]
extern crate clap;

use std::thread::{sleep, yield_now};
use std::time::Duration;

fn main() {
    let mut app = clap_app!(myapp =>
        (name: "Frame limiter test")
        (version: env!("CARGO_PKG_VERSION"))
        (author: "Pablo Marcos Oltra")
        (about: "Test to benchmark unlimited, yield and sleep frame limiter strategies")
        (@arg sleep: -s --sleep "sleep in the main loop")
        (@arg sleep_time: --sleep_time +takes_value "specify time to sleep (in ms)" )
        (@arg yield: -y --yield "yield_now in the main loop")
        (@arg unlimited: -u --unlimited "empty loop that consumes 1 logical core")
        (@arg cpu: -c --cpu "hog the CPU by creating threads with empty loop. By default it spawns 1 thread per logical core")
        (@arg threads: -t --threads +takes_value "number of threads for the CPU hog mode")
    );

    if std::env::args().len() == 1 {
        app.print_help().unwrap();
        println!();
        std::process::exit(-1);
    }

    let matches = app.get_matches();

    if matches.is_present("sleep") {
        let sleep_time = matches.value_of("sleep_time").unwrap_or("0").parse::<u64>().unwrap();
        println!("sleep({} ms) mode...", sleep_time);
        loop { sleep(Duration::from_millis(sleep_time)); }
    } else if matches.is_present("yield") {
        println!("yield mode...");
        loop { yield_now(); }
    } else if matches.is_present("unlimited") {
        println!("unlimited mode...");
        loop {}
    } else {
        let logical_cores = num_cpus::get().to_string();
        let num_threads = matches.value_of("threads").unwrap_or(&logical_cores);
        let num_threads = num_threads.parse::<usize>().unwrap();
        println!("CPU hog: about to create {} threads...", num_threads);

        let mut threads = Vec::with_capacity(num_threads);
        for i in 0..num_threads {
            println!("Spawning thread {}", i);
            threads.push(std::thread::spawn(move || {
                println!("Thread {} started", i);
                loop {}
            }));
        }

        for thread in threads {
            thread.join().unwrap();
        }
    }
}
