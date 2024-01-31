use std::{collections::HashSet, net::{Ipv4Addr, TcpStream, ToSocketAddrs}, time::Duration};
use chrono::Local;
use clap::{command, Arg};
use tz::TimeZone;
use colored::*;

fn parse_args(target: &str, ports: &str, timeout: &str) -> (String, Vec<u16>, String) {
    let mut ip_addr = String::new();
    let mut prt = Vec::new();
    let mut tout = String::new();

    match target.parse::<Ipv4Addr>() {
        Ok(ip) => {
            ip_addr = ip.to_string();
            let date = Local::now().format("%Y-%m-%d %H:%M:%S");
            let tz = TimeZone::local()
                .expect("Failed to get local timezone");
            let tz_name = tz
                .find_current_local_time_type()
                .expect("Failed to get local timezone name")
                .time_zone_designation();

            println!("Starting Rmap 0.1 at {} {}", date, tz_name);
            println!("Rmap scan report for {}", ip_addr);
        }
        Err(_) => eprint!("[{}] Introduce a valid IP address", "ERROR".red())
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

    match timeout.parse::<u16>() {
        Ok(t) => tout = t.to_string(),
        Err(_) => eprint!("[{}] Introduce a number", "ERROR".red())
    }

    (ip_addr, prt, tout)
}

fn scan_ports(ip: &str, ports: &[u16], timeout: String) -> (Vec<u16>, Vec<u16>) {
    let mut open_ports: Vec<u16> = Vec::new();
    let mut closed_ports: Vec<u16> = Vec::new();
    let mut tout: u64 = 0;

    match timeout.parse::<u64>() {
        Ok(t) => tout = t,
        Err(_) => {}
    }

    for &port in ports.iter() {
        let mut target = format!("{}:{}", ip, port)
            .to_socket_addrs()
            .unwrap();
        
        match TcpStream::connect_timeout(&target.next().unwrap(), Duration::from_secs(tout)) {
            Ok(_) => open_ports.push(port),
            Err(_) => closed_ports.push(port)
        }
    }

    (open_ports, closed_ports)
}

fn main() {
    let result_matches = command!()
        .name("rmap")
        .version("0.1")
        .author("rxfatalslash")
        .about("Port scanner written in Rust")
        .arg(
            Arg::new("TARGET")
            .help("IP address to scan")
            .index(1)
            .required(true)
        )
        .arg(
            Arg::new("PORTS")
            .help("Ports to scan")
            .short('p')
            .long("ports")
            .default_value("1-1024")
        )
        .arg(
            Arg::new("TIMEOUT")
            .help("Timeout")
            .short('t')
            .long("timeout")
            .default_value("10")
        )
        .get_matches();

    let target = result_matches.get_one::<String>("TARGET").unwrap();
    let ports = result_matches.get_one::<String>("PORTS").unwrap();
    let timeout = result_matches.get_one::<String>("TIMEOUT").unwrap();

    let (ip_addr, mut prt, tout) = parse_args(target, ports, timeout);

    if ports.contains("-") {
        let start_port = prt[0];
        let end_port = prt[1];

        for p in start_port..=end_port {
            prt.push(p);
        }
    }

    let no_dup: HashSet<u16> = prt.clone().into_iter().collect();
    let mut no_dup_sort: Vec<u16> = no_dup.into_iter().collect();
    no_dup_sort.sort();

    let (open_ports, _closed_ports) = scan_ports(&ip_addr, &prt, tout);

    println!(
        "{} closed ports",
        (no_dup_sort.len() - open_ports.len())
    );
    println!("PORT\tSTATE");
    for port in open_ports {
        println!("{}\t{}", port, "open".green());
    }
}
