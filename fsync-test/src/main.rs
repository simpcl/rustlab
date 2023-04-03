use clap::{Arg, ArgAction, ArgMatches};
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};

const BUF_SIZE: usize = 4096;

fn app_args() -> ArgMatches {
    clap::Command::new("fsync-test")
        .arg(
            Arg::new("file-path")
                .help("Sets the written file path")
                .long("file-path")
                .short('f')
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("file-size")
                .help("Sets the written file size (MB)")
                .long("file-size")
                .short('s')
                .value_parser(clap::value_parser!(u64))
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("interval")
                .help("Sets the written interval (ms)")
                .long("interval")
                .short('i')
                .value_parser(clap::value_parser!(u64))
                .action(ArgAction::Set),
        )
        .get_matches()
}

fn main() -> std::io::Result<()> {
    let matches = app_args();
    let file_path = matches.get_one::<String>("file-path").unwrap();
    let file_size_mb = *matches.get_one::<u64>("file-size").unwrap();
    let interval = *matches.get_one::<u64>("interval").unwrap();

    println!("file path: {}", file_path);
    println!("file size: {}MB", file_size_mb);
    println!("interval {}s", interval);

    //let mut f = File::options().append(true).create(true).open(file_path)?;
    let mut f = File::create(file_path)?;
    let mut buffer = [0u8; BUF_SIZE];
    let file_size = file_size_mb * 1024 * 1024;
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

    //Ok(())
}
