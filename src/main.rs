#![allow(unused_must_use)]
use crate::config::{ADDRESS, ADDRESS_PORT};
use crate::logger::start_input_handler;
use crate::network::network_server_start;
use fern::colors::Color;
use std::error::Error;
use std::process;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::SystemTime;
use std::{fmt, thread};
use utils::logger;

// Use a macros from serde(Serialize and Deserialize), log(Logging) and lazy_static(Global variables)
#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

pub mod config;
pub mod network;
pub mod utils;

// Main function of application
fn main() {
    let start = SystemTime::now();
    // Initialize logger
    println!("Starting ULE v1.0.0...");
    if let Err(err) = logger::setup_logger() {
        eprintln!("Failed to initialize logger: {}", err);
        process::exit(1);
    }
    // Creating channel for multithreading communication with main's thread and network's thread
    let (tx, rx) = channel::<bool>();
    // Generate server's address and make it accessible with thread safe
    let address = Arc::new(String::from(format!(
        "{}:{}",
        ADDRESS,
        ADDRESS_PORT.to_string()
    )));
    // Start network in another thread
    thread::spawn({
        let address = address.to_string();
        move || {
            // Start network
            // If failed to start when return error
            if let Err(err) = network_server_start(address, &tx) {
                error!("{}", err);
                tx.send(false);
            }
        }
    });
    // Wait for status from server's network
    if rx.recv().unwrap_or(false) {
        // If Server successful started
        info!("Server started at {}", address);
        // Showing about the full launch and showing the time to start
        {
            let elapsed = start.elapsed().unwrap();
            info!(
                "The server was successfully started in {}",
                if elapsed.as_secs() >= 1 {
                    format!("{}s", elapsed.as_secs())
                } else if elapsed.as_millis() >= 1 {
                    format!("{}ms", elapsed.as_millis())
                } else {
                    format!("{}ns", elapsed.as_nanos())
                }
            );
            drop(elapsed);
            drop(start);
        };
    } else {
        // If Failed to start Server
        error!("Failed to start server on {}.", address);
        process::exit(1);
    }
    // Remove channel
    std::mem::drop(rx);
    // Start console input handler(input commands)
    start_input_handler();
}

// Custom error(yes, not std::io:Error)
#[derive(Debug)]
pub struct SimpleError(String, Option<std::io::Error>);

impl Error for SimpleError {}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Check is error provided
        if self.1.is_some() {
            write!(f, "{}: {:?}", self.0, self.1)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

// Custom Result with custom Error
pub type SResult<T> = Result<T, SimpleError>;
