use num_cpus;

const DEFAULT_NUM_THREADS: usize = 1;
const DEFAULT_BATCH_SIZE: usize = 1_000_000;

pub struct Config {
    pub num_threads: usize,
    pub batch_size: usize,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let mut num_threads = DEFAULT_NUM_THREADS;
        let mut batch_size = DEFAULT_BATCH_SIZE;

        for (idx, key) in args.iter().enumerate() {
            match key.as_str() {
                "-t" => {
                    let val = args[idx + 1].parse::<isize>().unwrap();
                    if val > 0 {
                        num_threads = val as usize;
                    } else {
                        num_threads = num_cpus::get();
                    }
                }
                "-b" => {
                    let val = args[idx + 1].parse::<isize>().unwrap();
                    if val > 0 {
                        batch_size = val as usize;
                    }
                }
                _ => continue,
            }
        }

        Config {
            num_threads: num_threads,
            batch_size: batch_size,
        }
    }
}