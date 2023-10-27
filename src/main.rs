use std::io::{self};
use std::time::Duration;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use fastping_rs::PingResult::{Idle, Receive};
use fastping_rs::Pinger;

fn ping() {
        pretty_env_logger::init();
    let (pinger, results) = match Pinger::new(None, Some(56)) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}", e),
    };
    pinger.add_ipaddr("8.8.8.8");
    pinger.add_ipaddr("1.1.1.1");
    // pinger.add_ipaddr("7.7.7.7");
    // pinger.add_ipaddr("2001:4860:4860::8888");
    pinger.run_pinger();
    loop {
        match results.recv() {
            Ok(result) => match result {
                Idle { addr } => {
                    error!("Idle Address {}.", addr);
                }
                Receive { addr, rtt } => {
                    info!("Receive from Address {} in {:?}.", addr, rtt);
                }
            },
            Err(_) => panic!("Worker threads disconnected before the solution was found!"),
        }
    }
}

fn main() {
    loop {
        println!("Please choose an option:");
        println!("1. Ping");
        println!("0. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "1" => ping(),
            "0" => break,
            _ => println!("Invalid input"),
        }
    }
}
