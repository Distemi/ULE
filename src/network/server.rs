use crate::network::handler::{handshaking, status_handler};
use crate::network::network_client::ConnectionType::HANDSHAKING;
use crate::network::network_client::NetworkClient;
use ahash::AHashMap;
use mio::net::TcpListener;
use mio::{Events, Interest, Poll, Token};
use std::io;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::time::Duration;

// Declare global variables
lazy_static! {
    // Server need to shut down? (true - yes, needs to shutdown network server).
    pub static ref SHUTDOWN_SERVER: Mutex<bool> = Mutex::new(false);
    // Server's works status.
    pub static ref NET_SERVER_WORKS: Mutex<bool> = Mutex::new(true);
}

// Server's Token(ID)
const SERVER: Token = Token(0);

// Next Token
fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}

// Start a network server
pub fn network_server_start(address: String, tx: &Sender<bool>) -> std::io::Result<()> {
    // Creating Network Pool
    let mut poll = Poll::new()?;
    // Creating Network Events Pool
    let mut events = Events::with_capacity(128);
    // Converting String's address to SocketAddr
    let addr = address.parse().unwrap();
    // Starting a Network Listener
    let mut server = TcpListener::bind(addr)?;
    // Register server's Token
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    // Creating a list of connections
    let mut connections: AHashMap<Token, NetworkClient> = AHashMap::new();
    // Creating a variable with latest token.
    let mut unique_token = Token(SERVER.0 + 1);
    // Send over the channel that the server has been successfully started
    tx.send(true);

    // Network Events getting timeout
    let timeout = Some(Duration::from_millis(10));
    // Infinity loop(while true) to handing events
    loop {
        // Checks whether it is necessary to shutdown the network server
        if *SHUTDOWN_SERVER.lock().unwrap() {
            *NET_SERVER_WORKS.lock().unwrap() = false;
            info!("Network Server Stopped!");
            return Ok(());
        }
        // Getting a events from pool to event's pool with timeout
        poll.poll(&mut events, timeout)?;
        // Handing a events
        for event in events.iter() {
            // Handing event by token
            match event.token() {
                // If it server's event
                // Reading a all incoming connection
                SERVER => loop {
                    // Accepting connection
                    let (mut connection, _) = match server.accept() {
                        // If successful
                        Ok(v) => v,
                        // If not exists incoming connection
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                            break;
                        }
                        // If failed to get incoming connection
                        Err(e) => {
                            return Err(e);
                        }
                    };

                    // Generating new token for this connection
                    let token = next(&mut unique_token);
                    // Registering connection with token
                    poll.registry().register(
                        &mut connection,
                        token,
                        Interest::READABLE.add(Interest::WRITABLE),
                    )?;
                    // Pushing connection into connection's list
                    connections.insert(
                        token,
                        NetworkClient {
                            stream: connection,
                            conn_type: HANDSHAKING,
                        },
                    );
                },
                // Handing event from client
                token => {
                    // Handing event by connection's stage
                    let done = if let Some(connection) = connections.get_mut(&token) {
                        let m = match &connection.conn_type {
                            HANDSHAKING => handshaking,
                            _ => status_handler,
                        };
                        // Trying to handing
                        m(connection, &event).unwrap_or(false)
                    } else {
                        false
                    };
                    // If needs to close connection - removing from list, unregister and close connection's stream
                    if done {
                        if let Some(mut connection) = connections.remove(&token) {
                            poll.registry().deregister(&mut connection.stream)?;
                            connections.remove(&token);
                        }
                    }
                }
            }
        }
    }
}
