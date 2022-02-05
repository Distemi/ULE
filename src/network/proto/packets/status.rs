use crate::config::PROTOCOL_VERSION;
use crate::network::proto::packet_write::PacketWriter;
use crate::utils::chat::ChatMessage;

// Structs for status MOTD response
#[derive(Debug, Serialize)]
pub struct ListPingResponse {
    pub version: ListPingResponseVersion,
    pub players: ListPingResponsePlayers,
    pub description: ChatMessage,
}

#[derive(Debug, Serialize)]
pub struct ListPingResponseVersion {
    pub name: String,
    pub protocol: u32,
}

#[derive(Debug, Serialize)]
pub struct ListPingResponsePlayers {
    pub max: u32,
    pub online: u32,
    pub sample: Vec<ListPingResponsePlayerSample>,
}

#[derive(Debug, Serialize)]
pub struct ListPingResponsePlayerSample {
    pub name: String,
    pub id: String,
}
/// Build packet's bytes as result
pub fn create_server_list_ping_response() -> Vec<u8> {
    // Initialize empty byte's vector
    let mut bytes = Vec::new();
    // Generating String and convert to bytes.
    // String generated as JSON by serde and serde_json libraries
    bytes.write_string(
        serde_json::to_string(&ListPingResponse {
            version: ListPingResponseVersion {
                name: String::from("ULE"),
                protocol: PROTOCOL_VERSION,
            },
            players: ListPingResponsePlayers {
                max: 10,
                online: 0,
                sample: vec![],
            },
            // Some clients can read colors and so on without convert into JSON
            description: ChatMessage::str("&a&lHello!"),
        })
        .unwrap(),
    );
    // Build completed packet. Server List Ping - PacketID is 0x00
    bytes.create_packet(0x00)
}
