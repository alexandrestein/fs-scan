use std::env;
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc;
use std::thread;
use std::{fs, io};

// use std::sync::Mutex;

use num_cpus;

fn main() -> io::Result<()> {
    let mut path = String::from(".");
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        println!("The path is {}", args[1]);
        path = String::from(&args[1]);
    };
    let num = num_cpus::get();
    println!("{}", num);

    let (tx, rx):(mpsc::Sender<usize>, mpsc::Receiver<usize>) = mpsc::channel();
    let mut atomic_num_of_running_tasks = AtomicUsize::new(0);

    let mut res = build_result();

    let guard = thread::scoped(|| {
        res = handle_dir(&atomic_num_of_running_tasks, path.as_str(), res);
    });


    println!("Files -> {}", nice_number(res.files));
    println!("Directories -> {}", nice_number(res.directories));
    println!("Less than 4K -> {}", nice_number(res.less_than_4_k));
    println!(
        "Between 4K and 16K -> {}",
        nice_number(res.between_4_k_16_k)
    );
    println!(
        "Between 16K and 64K -> {}",
        nice_number(res.between_16_k_64_k)
    );
    println!(
        "Between 64K and 128K -> {}",
        nice_number(res.between_64_k_128_k)
    );
    println!(
        "Between 128K and 256K -> {}",
        nice_number(res.between_128_k_256_k)
    );
    println!(
        "Between 256K and 512K -> {}",
        nice_number(res.between_256_k_512_k)
    );
    println!(
        "Between 512K and 1M -> {}",
        nice_number(res.between_512_k_1_m)
    );
    println!(
        "Between 1M and 10M -> {}",
        nice_number(res.between_1_m_10_m)
    );
    println!(
        "Between 10M and 100M -> {}",
        nice_number(res.between_10_m_100_m)
    );
    println!(
        "Between 100M and 1G -> {}",
        nice_number(res.between_100_m_1_g)
    );
    println!("More than 1G -> {}", nice_number(res.more_than_1_g));

    Ok(())
}

fn nice_number(input: usize) -> String {
    if input < 1_000 {
        return format!("{:?}", input);
    } else if input < 1_000_000 {
        return format!("{:?}K ({:?})", input / 1_000, input);
    } else {
        return format!("{:?}M ({:?})", input / 1_000_000, input);
    }
}

fn handle_dir_multi(mut nb: &AtomicUsize, path: &str, tx: mpsc::Sender<usize>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        res.directories = res.directories + 1;
                        match entry.path().to_str() {
                            Some(s) => {
                                res = handle_dir(nb, s, res);
                            }
                            None => println!("no regular path {:?}", entry.path()),
                        }
                        // println!("{:?}: is dir", entry.path());
                    }
                    if metadata.is_file() {
                        handle_file(metadata.len(), &mut res);
                    }
                } else {
                    println!("Couldn't get file type for {:?}", entry.path());
                }
            }
        }
    }
}

fn handle_dir(mut nb: &AtomicUsize, path: &str, mut res: Result) -> Result {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        res.directories = res.directories + 1;
                        match entry.path().to_str() {
                            Some(s) => {
                                res = handle_dir(nb, s, res);
                            }
                            None => println!("no regular path {:?}", entry.path()),
                        }
                        // println!("{:?}: is dir", entry.path());
                    }
                    if metadata.is_file() {
                        handle_file(metadata.len(), &mut res);
                    }
                } else {
                    println!("Couldn't get file type for {:?}", entry.path());
                }
            }
        }
    }

    res
}

// fn handle_file_multi(len: u64, chan: ) {}
fn handle_file(len: u64, res: &mut Result) {
    if len < 4_000 {
        res.less_than_4_k = res.less_than_4_k + 1;
    } else if len < 16_000 {
        res.between_4_k_16_k = res.between_4_k_16_k + 1;
    } else if len < 64_000 {
        res.between_16_k_64_k = res.between_16_k_64_k + 1;
    } else if len < 128_000 {
        res.between_64_k_128_k = res.between_64_k_128_k + 1;
    } else if len < 256_000 {
        res.between_128_k_256_k = res.between_128_k_256_k + 1;
    } else if len < 512_000 {
        res.between_256_k_512_k = res.between_256_k_512_k + 1;
    } else if len < 1_000_000 {
        res.between_512_k_1_m = res.between_512_k_1_m + 1;
    } else if len < 10_000_000 {
        res.between_1_m_10_m = res.between_1_m_10_m + 1;
    } else if len < 100_000_000 {
        res.between_10_m_100_m = res.between_10_m_100_m + 1;
    } else if len < 1_000_000_000 {
        res.between_100_m_1_g = res.between_100_m_1_g + 1;
    } else {
        res.more_than_1_g = res.more_than_1_g + 1;
    }
    res.files = res.files + 1;
}

struct Result {
    files: usize,
    directories: usize,
    less_than_4_k: usize,
    between_4_k_16_k: usize,
    between_16_k_64_k: usize,
    between_64_k_128_k: usize,
    between_128_k_256_k: usize,
    between_256_k_512_k: usize,
    between_512_k_1_m: usize,
    between_1_m_10_m: usize,
    between_10_m_100_m: usize,
    between_100_m_1_g: usize,
    more_than_1_g: usize,
}
fn build_result() -> Result {
    Result {
        files: 0,
        directories: 0,

        less_than_4_k: 0,
        between_4_k_16_k: 0,
        between_16_k_64_k: 0,
        between_64_k_128_k: 0,
        between_128_k_256_k: 0,
        between_256_k_512_k: 0,
        between_512_k_1_m: 0,
        between_1_m_10_m: 0,
        between_10_m_100_m: 0,
        between_100_m_1_g: 0,
        more_than_1_g: 0,
    }
}
