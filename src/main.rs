use std::env;
use std::io;
use std::time::{Instant};

fn main() {
    let elements: u32;
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let trimmed = args[1].trim();
        match trimmed.parse::<u32>() {
            Ok(i) => elements = i,
            Err(..) => {
                eprintln!("Invalid argument - not an integer: {}", trimmed);
                return;
            },
        };
    } else {
        println!("Type number of Fubonacci elements to be printed:");
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => elements = i,
            Err(..) => {
                eprintln!("This was not an integer: {}", trimmed);
                return;
            },
        };

        if elements < 1 {
            eprintln!("this was not an integer: {}", trimmed);
            return;
        }
    }


    let start = Instant::now();

    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut tmp: u64;
    
    println!("{}: {}", 0, a);
    for i in 1..elements+1 {
        tmp = b;
        b = a + b;
        a = tmp;
        println!("{}: {}", i, a);
    }

    let duration = start.elapsed();

    println!("Time elapsed: {:?}", duration);
}
