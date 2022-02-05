use crate::{SResult, SimpleError};
use mio::net::TcpStream;
use std::io::{ErrorKind, Read};

// Connection's types
pub enum ConnectionType {
    HANDSHAKING,
    STATUS,
}

// Network-base client
pub struct NetworkClient {
    pub stream: TcpStream,
    pub conn_type: ConnectionType,
}

// Declare functions
impl NetworkClient {
    // Function for reading input bytes
    pub fn read(&mut self) -> SResult<(bool, Vec<u8>)> {
        // Creating a buffer up to 4KB information
        let mut bytes = vec![0; 4096];
        // Reading a bytes
        let (ok, err) = match self.stream.read(&mut bytes) {
            // Connection closed. Why exists?
            Ok(0) => (false, None),
            // Getting byte's length and resize buffer
            Ok(n) => {
                bytes.resize(n, 0);
                (true, None)
            }
            // Connection don't has a input bytes
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (false, None),
            // Failed to read bytes
            Err(err) => (false, Some(err)),
        };
        // If Error exists when returning a error
        if err.is_some() {
            return Err(SimpleError(String::from("Failed to read packet"), err));
        }
        // Returning a result of reading
        Ok((ok, bytes))
    }
}
