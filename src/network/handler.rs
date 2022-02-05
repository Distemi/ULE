use crate::network::network_client::ConnectionType::STATUS;
use crate::network::network_client::NetworkClient;
use crate::network::proto::packets::handshaking::read_handshake_packet;
use crate::network::proto::packets::status::create_server_list_ping_response;
use crate::network::proto::PacketReader;
use crate::SResult;
use mio::event::Event;
use std::io::Write;

// Handshaking connection's stage
pub fn handshaking(conn: &mut NetworkClient, event: &Event) -> SResult<bool> {
    // Checking if we can read the package
    if !event.is_readable() {
        return Ok(false);
    }
    // Reading packet
    let handshake = read_handshake_packet(conn);
    // Checking if is error
    if handshake.is_err() {
        return Ok(true);
    }
    // Getting results
    let (_, _, _, next_state) = handshake.unwrap();
    // Change types
    conn.conn_type = match next_state {
        1 => STATUS,
        _ => STATUS,
    };
    Ok(false)
}

// Status connection's stage
pub fn status_handler(conn: &mut NetworkClient, event: &Event) -> SResult<bool> {
    // Checking if we can read and write
    if !event.is_readable() || !event.is_writable() {
        return Ok(false);
    }
    // Getting a input's bytes
    let (ok, p, err) = match conn.read() {
        Ok((ok, p)) => (ok, Some(p), None),
        Err(err) => (false, None, Some(err)),
    };
    // Checking if a read or not
    if !ok {
        return Ok(err.is_some());
    }
    // Packet's bytes
    let mut p: Vec<u8> = p.unwrap();
    // Cloning bytes(for ping-pong)
    let bytes = p.clone();
    // Reading a packet's length(and remove...) and PacketID
    let (_, pid) = p.read_base()?;
    match pid {
        // Is Ping List
        0x00 => {
            drop(bytes);
            conn.stream.write_all(&*create_server_list_ping_response());
        }
        // Is Ping-Pong
        0x01 => {
            conn.stream.write_all(bytes.as_slice());
            match conn.stream.peer_addr() {
                Ok(v) => info!("Server pinged from {}", v),
                Err(_) => {
                    info!("Server pinged.")
                }
            }
        }
        _ => {}
    }
    Ok(false)
}
