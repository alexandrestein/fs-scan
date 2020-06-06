use std::env;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time;
use std::{fs, io};

use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use num_cpus;

enum ResponseType {
    File,
    Dir,
    DoneDir,
}
struct ChanResponse {
    t: ResponseType,
    path: PathBuf,
    len: u64,
}
fn build_dir_chan(path: PathBuf) -> ChanResponse {
    ChanResponse {
        t: ResponseType::Dir,
        path,
        len: 0,
    }
}
fn build_dir_chan_done() -> ChanResponse {
    ChanResponse {
        t: ResponseType::DoneDir,
        path: PathBuf::new(),
        len: 0,
    }
}
fn build_file_chan(size: u64) -> ChanResponse {
    ChanResponse {
        t: ResponseType::File,
        path: PathBuf::new(),
        len: size,
    }
}

fn main() -> io::Result<()> {
    let mut path = PathBuf::from(".");
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        println!("The path is {}", args[1]);
        path = PathBuf::from(&args[1]);
    };

    let mut res = build_result();

    // build channel
    let (sender, receiver) = channel();
    let saved_num_cpu = num_cpus::get() * 4;

    let bar = ProgressBar::new(saved_num_cpu as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{per_sec}] {elapsed} {bar:.cyan/blue} {pos:>3}/{len:3} {msg}")
            .progress_chars("##-"),
    );

    // Start scanning at the given path
    handle_dir(path, sender.clone(), &bar);

    let cloned_sender_again = sender.clone();
    let mut running_thread = 0;
    let mut dir_queue = Vec::new();
    let starting_point = time::Instant::now();

    // Handle responses
    for received in receiver {
        bar.set_message(&format!(
            "files scanned {} and dirs in queue {}",
            &res.files, &dir_queue.len()
        ));
        // Check the type of the given element
        match received.t {
            // If Dir
            ResponseType::Dir => {
                res.directories += 1;
                // Check if the number of running thread is not too height
                if running_thread >= saved_num_cpu {
                    // If it's over four times the number of CPU than the folder is saved into a queue
                    dir_queue.push(received);
                } else {
                    // No problem with too much concurrency, so let's run the scan right away
                    running_thread += 1;
                    bar.set_position(running_thread as u64);
                    handle_dir(received.path, cloned_sender_again.clone(), &bar);
                }
            }
            // If this signal a directory scan terminated
            ResponseType::DoneDir => {
                // The process is done
                // Break the loop to display the results
                if running_thread == 0 {
                    bar.set_message(&format!("Total file scanned {}", &res.files));
                    break;
                }
                match dir_queue.pop() {
                    Some(dir) => {
                        handle_dir(dir.path, cloned_sender_again.clone(), &bar);
                    }
                    None => {
                        running_thread -= 1;
                        bar.set_position(running_thread as u64);
                    }
                };
            }
            // If File
            ResponseType::File => {
                handle_file(received.len, &mut res);
            }
        }
    }
    bar.finish();

    println!("Scan took {}", HumanDuration(starting_point.elapsed()));
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

fn handle_dir(path: PathBuf, ch: Sender<ChanResponse>, bar: &ProgressBar) {
    match fs::read_dir(&path) {
        Ok(entries) => {
            let ch = ch.clone();
            let bar = bar.clone();
            thread::spawn(move || {
                for entry in entries {
                    match entry {
                        Ok(entry) => match entry.metadata() {
                            Ok(metadata) => {
                                if metadata.is_dir() {
                                    let ch = ch.clone();
                                    ch.send(build_dir_chan(entry.path())).unwrap();
                                } else if metadata.is_file() {
                                    ch.send(build_file_chan(metadata.len())).unwrap();
                                }
                            }
                            Err(err) => {
                                bar.println(format!(
                                    "Couldn't get file metadata for {:?}: {}",
                                    entry.path(),
                                    err
                                ));
                            }
                        },
                        Err(err) => {
                            bar.println(format!("warning 1 {}", err));
                        }
                    }
                }
                // Notify the end of the thread
                ch.send(build_dir_chan_done()).unwrap();
            });
        }
        Err(err) => {
            bar.println(format!("warning 0 {} {:?}", err, &path));
            // Notify the end of the thread
            ch.send(build_dir_chan_done()).unwrap();
        }
    }
}

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
