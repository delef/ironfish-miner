pub struct Config {
    pub threads: usize,
    pub batch_size: usize,
}

impl Config {
    pub new(args: &[String]) {
        let threads = [];

        for (n,v) in args {
            println!("{}: {}", n,v);
        }
    }
}