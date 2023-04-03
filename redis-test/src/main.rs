use clap::{Arg, ArgAction, ArgMatches};
use redis::Commands;
use std::{thread, time};

fn app_args() -> ArgMatches {
    clap::Command::new("redis-test")
        .arg(
            Arg::new("cmd")
                .help("Sets cmd (set|get|del)")
                .long("cmd")
                .required(true)
                .value_parser(["set", "get", "del"])
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("count")
                .help("Sets count of requests")
                .long("count")
                .short('c')
                .value_parser(clap::value_parser!(u64))
                .default_value("10")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("interval")
                .help("Sets interval in milliseconds for all requests (ms)")
                .long("interval")
                .short('i')
                .value_parser(clap::value_parser!(u64))
                .default_value("10")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("data-size")
                .help("Sets data size")
                .long("data-size")
                .value_parser(clap::value_parser!(usize))
                .default_value("100")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("nodes")
                .help("Sets the redis connection string (redis://:password@127.0.0.1:6379/,redis://:password@127.0.0.1:6380)")
                .long("nodes")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("cluster")
                .help("Sets the redis cluster mode")
                .long("cluster")
                .action(ArgAction::SetTrue),
        )
        .get_matches()
}

fn main() -> redis::RedisResult<()> {
    let matches = app_args();

    let cmd = matches.get_one::<String>("cmd").unwrap();
    let count = *matches.get_one::<u64>("count").unwrap();
    let interval = *matches.get_one::<u64>("interval").unwrap();
    let data_size = *matches.get_one::<usize>("data-size").unwrap();
    let nodes = matches.get_one::<String>("nodes").unwrap();
    let is_cluster = matches.get_flag("cluster");

    println!("cmd: {}", cmd);
    println!("count: {}", count);
    println!("interval: {}", interval);
    println!("data-size: {}", data_size);
    println!("nodes: {}", nodes);
    println!("is_cluster: {}", is_cluster);

    let key = String::from("redis-test");
    let value: Vec<u8> = vec![b'a'; data_size];

    if !is_cluster {
        let client = redis::Client::open(&nodes[..]).unwrap();
        let mut con = client.get_connection().unwrap();
        //con.ping()?;
        let _: () = redis::cmd("PING").query(&mut con).unwrap();
        for _i in 0..count {
            let begin_time = time::SystemTime::now();
            if cmd == "set" {
                let _: () = con.set(key.as_bytes(), &value[..])?;
            } else if cmd == "get" {
                let _: String = con.get(key.as_bytes())?;
            } else if cmd == "del" {
                con.del(key.as_bytes())?;
            }
            let end_time = time::SystemTime::now();
            let difference = end_time.duration_since(begin_time).unwrap();
            println!(
                "{} {} duration: {} us",
                end_time
                    .duration_since(time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                cmd,
                difference.as_micros()
            );
            if interval > 0 {
                thread::sleep(time::Duration::from_millis(interval));
            }
        }
    } else {
        let tokens: Vec<&str> = nodes.split(",").collect();
        /*let nodes = vec![
            "redis://127.0.0.1:30001/",
            "redis://127.0.0.1:30002/",
            "redis://127.0.0.1:30003/",
            "redis://127.0.0.1:30004/",
            "redis://127.0.0.1:30005/",
            "redis://127.0.0.1:30006/",
        ];*/
        let client = redis::cluster::ClusterClient::new(tokens).unwrap();
        let mut con = client.get_connection().unwrap();
        //con.ping()?;
        let _: () = redis::cmd("PING").query(&mut con).unwrap();
        for _i in 0..count {
            let begin_time = time::SystemTime::now();
            if cmd == "set" {
                let _: () = con.set(key.as_bytes(), &value[..])?;
            } else if cmd == "get" {
                let _: String = con.get(key.as_bytes())?;
            } else if cmd == "del" {
                con.del(key.as_bytes())?;
            }
            let end_time = time::SystemTime::now();
            let difference = end_time.duration_since(begin_time).unwrap();
            println!(
                "{} {} duration: {} us",
                end_time
                    .duration_since(time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                cmd,
                difference.as_micros()
            );
            if interval > 0 {
                thread::sleep(time::Duration::from_millis(interval));
            }
        }
    }
    Ok(())
}
