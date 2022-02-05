use crate::network::{NET_SERVER_WORKS, SHUTDOWN_SERVER};
use std::time::Duration;
use std::{io, process, thread};

// Loop for handling input
pub fn start_input_handler() -> std::io::Result<()> {
    // Input buffer
    let mut inp = String::new();
    // STDIN - os input
    let stdin = io::stdin();
    // loop for infinity handling
    loop {
        // Before write buffer we need to clear buffer
        inp.clear();
        // Reading a line
        stdin.read_line(&mut inp)?;
        // Clearing input's buffer
        inp = inp.replace("\n", "");
        // Simple realisation of stop command, but in updates be removed from here in another place
        if inp.starts_with("stop") {
            // Sending status to shutdown network server
            *SHUTDOWN_SERVER.lock().unwrap() = true;
            info!("Stopping server...");
            // Running process killing in 6 secs if failed to common shutdown
            thread::spawn(|| {
                thread::sleep(Duration::from_secs(6));
                process::exit(0);
            });
            // Waiting for shutdown network's server
            loop {
                if *NET_SERVER_WORKS.lock().unwrap() == true {
                    thread::sleep(Duration::from_millis(25));
                } else {
                    break;
                }
            }
            // Disabling the input
            return Ok(());
        }
        // If it's not stop command - when display buffer, but in updates be removed
        info!("Entered: {}", inp);
    }
}
