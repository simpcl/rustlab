use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::{thread, time};

const BUF_SIZE: usize = 4096;

fn usage() {
    println!("./fsync-test [file path] [fize size (MB)] [interval]")
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        usage();
        //return Err(Error::new(ErrorKind::Other, "abc"));
        return Ok(());
    }

    let file_path = &args[1];
    let file_size = args[2].parse::<u64>().unwrap();
    let interval = args[3].parse::<u64>().unwrap();

    println!("file path: {}", file_path);
    println!("file size: {}MB", file_size);
    println!("interval {}s", interval);

    //let mut f = File::options().append(true).create(true).open(file_path)?;
    let mut f = File::create(file_path)?;
    let mut buffer = [0u8; BUF_SIZE];
    let file_size = file_size * 1024 * 1024;
    let buffer_size: u64 = u64::try_from(BUF_SIZE).unwrap();

    let mut w_pos: u64 = 0;
    loop {
        let begin_time = time::SystemTime::now();
        f.write_all(&mut buffer)?;
        //f.flush()?;
        f.sync_data()?;
        let end_time = time::SystemTime::now();
        let difference = end_time.duration_since(begin_time).unwrap();
        println!(
            "{} duration: {} us",
            end_time
                .duration_since(time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            difference.as_micros()
        );
        w_pos += buffer_size;
        if w_pos >= file_size {
            f.seek(std::io::SeekFrom::Start(0))?;
            w_pos = 0;
        }
        thread::sleep(time::Duration::from_secs(interval));
        //std::io::stdout().flush()?;
    }

    Ok(())
}
