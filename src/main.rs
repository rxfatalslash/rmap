use std::{net::{Ipv4Addr, TcpStream, ToSocketAddrs}, time::Duration};
use chrono::Local;
use rand::Rng;
use clap::{command, Arg};
use tz::TimeZone;
use colored::*;

fn opt_to_str(opt: Option<&String>) -> String {
    match opt {
        Some(s) => s.clone(),
        None => String::new()
    }
}

fn parse_args(target: &str, ports: &str, timeout: Option<&String>) -> (Vec<String>, Vec<u16>, String) {
    let mut ip_addr = Vec::new();
    let mut _ip_target = Vec::new();
    let mut prt = Vec::new();
    let mut tout = String::new();
    let ti = opt_to_str(timeout);
    let parts: Vec<&str> = target.split(|c| c == '.').collect();

    if target.contains(",") {
        _ip_target = target.split(|c| c == ',')
            .map(|x| x.parse::<String>().unwrap())
            .collect();
    } else if let Some(last_part) = parts.last() {
        match last_part {
            &"0" => {
                let p: Vec<&str> = parts.iter().take(3).cloned().collect();
                let new_ip = p.join(".");

                for i in 1..=255 {
                    let t = format!("{}.{}", new_ip, i);

                    _ip_target.push(t);
                }
            }
            _ => {}
        }
    }

    for t in _ip_target {
        match t.parse::<Ipv4Addr>() {
            Ok(ip) => ip_addr.push(ip.to_string()),
            Err(_) => eprint!("[{}] Enter a valid IP address", "ERROR".red())
        }
    }

    if ports.contains("-") {
        prt = ports.split(|c| c == '-')
            .map(|x| x.parse::<u16>().unwrap())
            .collect();
    } else if ports.contains(",") {
        prt = ports.split(|c| c == ',')
            .map(|x| x.parse::<u16>().unwrap())
            .collect();
    } else if ports.contains("_") {
        for p in 1..=65535 {
            prt.push(p);
        }
    } else {
        match ports.parse::<u16>() {
            Ok(number) => prt = vec![number],
            Err(_) => eprintln!("[{}] Failed to parse ports", "ERROR".red())
        }
    }

    if timeout.is_some() {
        match ti.parse::<u16>() {
            Ok(t) => tout = t.to_string(),
            Err(_) => eprint!("[{}] Enter a number", "ERROR".red())
        }
    }

    (ip_addr, prt, tout)
}

fn scan_ports(ip: &[String], ports: &[u16], timeout: String) {
    let mut tout: u64 = 0;
    let mut op_p: Vec<u16> = Vec::new();
    let mut no_dup_sort: Vec<&u16> = ports.into_iter().collect();
    no_dup_sort.sort();
    let date = Local::now().format("%Y-%m-%d %H:%M:%S");
    let tz = TimeZone::local()
        .expect("Failed to get local timezone");
    let tz_name = tz
        .find_current_local_time_type()
        .expect("Failed to get local timezone name")
        .time_zone_designation();

    if timeout.is_empty() {
        let mut rng = rand::thread_rng();
        let rand_number: u64 = rng.gen_range(10..60);

        tout = rand_number;
    } else {
        match timeout.parse::<u64>() {
            Ok(t) => tout = t,
            Err(_) => {}
        }
    }

    println!("Starting Rmap 0.1.1 at {} {}", date, tz_name);

    for t in ip {
        let mut open_ports: Vec<u16> = Vec::new();
        let mut closed_ports: Vec<u16> = Vec::new();

        for &port in ports.iter() {
            let mut target = format!("{}:{}", t, port)
                .to_socket_addrs()
                .unwrap();
            
            match TcpStream::connect_timeout(&target.next().unwrap(), Duration::from_millis(tout)) {
                Ok(_) => {
                    open_ports.push(port);
                    op_p.push(port);
                },
                Err(_) => closed_ports.push(port)
            }
        }

        if open_ports.len() > 0 {
            println!("Rmap scan report for {}", t);

            println!(
                "{} closed ports",
                (no_dup_sort.len() - open_ports.len())
            );

            println!("PORT\tSTATE");
            for port in &open_ports {
                println!("{}\t{}", port, "open".green());
            }
        }
    }

    if op_p.len() == 0 {
        println!("No port open on the hosts");
    }
}

fn main() {
    let result_matches = command!()
        .name("rmap")
        .version("0.1.1")
        .author("rxfatalslash")
        .arg(
            Arg::new("TARGET")
            .help("IP address to scan, use , to scan one or more hosts, or enter an ip of type X.X.X.0 to scan all hosts on the network")
            .index(1)
            .required(true)
        )
        .arg(
            Arg::new("PORTS")
            .help("Ports to scan, use , to scan one or more ports, - to scan a range between this values, _ to scan the entire port range")
            .short('p')
            .long("ports")
            .default_value("1-1024")
        )
        .arg(
            Arg::new("TIMEOUT")
            .help("Timeout in milliseconds, the default value is a random number between 0 and 60")
            .short('t')
            .long("timeout")
        )
        .get_matches();

    let target = result_matches.get_one::<String>("TARGET").unwrap();
    let ports = result_matches.get_one::<String>("PORTS").unwrap();
    let timeout = result_matches.try_get_one::<String>("TIMEOUT").unwrap();

    let (ip_addr, mut prt, tout) = parse_args(target, ports, timeout);

    if ports.contains("-") {
        let start_port = prt[0];
        let end_port = prt[1];

        for p in start_port..=end_port {
            prt.push(p);
        }
    }

    scan_ports(&ip_addr, &prt, tout);
}
