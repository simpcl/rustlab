use clap::crate_version;
use clap::{App, Arg};
use redis::Commands;
use std::env;
use std::{thread, time};

fn app_args<'a>() -> clap::ArgMatches<'a> {
    App::new("redis-test")
        .version(crate_version!())
        .about("Redis testing application written in Rust")
        .arg(
            Arg::with_name("cluster")
                .help("Sets the redis cluster mode")
                .long("cluster")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("cmd")
                .help("Sets cmd (set|get|del)")
                .long("cmd")
                .required(true)
                .takes_value(true),
        )
        .arg(Arg::with_name("count").short("n").long("count").help("Sets count of requests").takes_value(true))
        .arg(Arg::with_name("interval").long("interval").help("Sets interval in milliseconds for all requests").takes_value(true))
        .arg(Arg::with_name("data-size").long("data-size").help("Sets data size").takes_value(true))
        .arg(
            Arg::with_name("nodes")
                .help("Sets the redis connection string (redis://:password@127.0.0.1:6379/,redis://:password@127.0.0.1:6380)")
                .required(true)
                .takes_value(true),
        )
        .get_matches()
}

fn main() -> redis::RedisResult<()> {
    let matches = app_args();
    let nodes = matches.value_of("nodes").unwrap();
    let is_cluster = matches.is_present("cluster");
    let count_str = matches.value_of("count").unwrap_or("10");
    let interval_str = matches.value_of("interval").unwrap_or("0");
    let cmd = matches.value_of("cmd").unwrap();
    let data_size_str = matches.value_of("data-size").unwrap_or("10");

    let count = count_str.parse::<u64>().unwrap();
    let interval = interval_str.parse::<u64>().unwrap();
    let data_size = data_size_str.parse::<usize>().unwrap();

    println!("cmd: {}", cmd);
    println!("count: {}", count);
    println!("is_cluster: {}", is_cluster);
    println!("nodes: {}", nodes);

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
