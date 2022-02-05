use crate::network::network_client::NetworkClient;
use crate::network::proto::PacketReader;
use crate::{SResult, SimpleError};
/// Trying to read [handshake](https://wiki.vg/index.php?title=Protocol&oldid=14204#Handshake) packet
pub fn read_handshake_packet(client: &mut NetworkClient) -> SResult<(u32, String, u16, u32)> {
    // Read bytes from client
    let (ok, p, err) = match client.read() {
        Ok((ok, p)) => (ok, Some(p), None),
        Err(err) => (false, None, Some(err)),
    };
    // If failed to read when...
    if !ok || err.is_some() {
        return Err(SimpleError(
            String::from("Failed to read packet"),
            if err.is_some() { err.unwrap().1 } else { None },
        ));
    }
    // Reading packet
    let mut p: Vec<u8> = p.unwrap();
    // Try to read Length and PacketID from packet(on handshaking stage only 0x00)
    p.read_base()?;
    // Reading version, address and etc.
    let ver = p.get_varint()? as u32;
    let address = p.get_string()?;
    let port = p.get_u16();
    let next_state = p.get_varint()? as u32;
    // States can be only 1 - status, 2 - play
    if next_state >= 3 {
        return Err(SimpleError(String::from("Invalid client"), None));
    }
    // Returning results
    Ok((ver, address, port, next_state))
}
