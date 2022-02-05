/// Writer [Vec] of bytes
pub trait PacketWriter {
    // 1-Byte
    fn write_u8(&mut self, value: u8);
    fn write_i8(&mut self, value: i8);
    // 2-Byte
    fn write_u16(&mut self, value: u16);
    fn write_i16(&mut self, value: i16);
    // 4-Byte
    fn write_varint(&mut self, value: i32);
    // 8-Byte
    fn write_i64(&mut self, value: i64);
    // Another
    fn write_vec_bytes(&mut self, bytes: Vec<u8>);
    fn write_string(&mut self, value: String);
    fn create_packet(&mut self, pid: i32) -> Vec<u8>;
}

impl PacketWriter for Vec<u8> {
    // Writing byte
    fn write_u8(&mut self, value: u8) {
        self.push(value);
    }

    // Writing byte
    fn write_i8(&mut self, value: i8) {
        self.push(value as u8)
    }

    // Writing 2-byte unsigned integer
    fn write_u16(&mut self, value: u16) {
        self.extend_from_slice(&value.to_be_bytes());
    }

    // Writing 2-byte unsigned integer
    fn write_i16(&mut self, value: i16) {
        self.extend_from_slice(&value.to_be_bytes());
    }

    // Writing bytes as VarInt
    fn write_varint(&mut self, mut value: i32) {
        // Bytes buffer
        let mut buf = vec![0u8; 1];
        // Byte's length
        let mut n = 0;
        // Converts value to bytes
        loop {
            // Break if it's limit
            if value <= 127 || n >= 8 {
                break;
            }
            // Pushing a byte to buffer
            buf.insert(n, (0x80 | (value & 0x7F)) as u8);
            // Moving value's bits on 7
            value >>= 7;
            value -= 1;
            n += 1;
        }
        // Pushing byte, because it lower that 256(<256)
        buf.insert(n, value as u8);
        n += 1;
        // Pushing converted bytes into byte's buffer
        self.extend_from_slice(&buf.as_slice()[..n])
    }

    // Writing Long ( 64-Bit Integer )
    fn write_i64(&mut self, value: i64) {
        self.extend_from_slice(value.to_be_bytes().as_slice())
    }

    // Alias of extend_from_slice, but works with Vec, not Slice
    fn write_vec_bytes(&mut self, mut bytes: Vec<u8>) {
        self.append(&mut bytes);
    }

    // Write String (VarInt as len and string's bytes)
    fn write_string(&mut self, value: String) {
        // Getting String as Bytes
        let bytes = value.as_bytes();
        // Writing to buffer a length as VarInt
        self.write_varint(bytes.len() as i32);
        // Writing to buffer a string's bytes
        self.extend_from_slice(bytes);
    }

    // Packet's base builder
    fn create_packet(&mut self, pid: i32) -> Vec<u8> {
        // Creating empty packet's buffer
        let mut packet = Vec::new();
        // Creating length's bytes buffer and fill it as VarInt
        let mut len_bytes: Vec<u8> = Vec::new();
        len_bytes.write_varint(pid);
        // Writing full packet's length(content + length's bytes)
        packet.write_varint((self.len() + len_bytes.len()) as i32);
        // Writing length bytes
        packet.extend_from_slice(len_bytes.as_slice());
        // Drop(Free) length bytes buffer
        drop(len_bytes);
        // Writing some packet's content
        packet.extend_from_slice(self.as_slice());
        // Returning result
        packet
    }
}
