use crate::{SResult, SimpleError};

/// Reader [Vec] of bytes
pub trait PacketReader {
    // 1-Byte
    fn get_u8(&mut self) -> u8;
    fn get_i8(&mut self) -> i8;
    // 2-Byte
    fn get_u16(&mut self) -> u16;
    fn get_i16(&mut self) -> i16;
    // 4-Byte
    fn get_varint(&mut self) -> SResult<i32>;
    // 8-Byte
    fn get_i64(&mut self) -> i64;
    // Another
    fn get_string(&mut self) -> SResult<String>;
    fn read_base(&mut self) -> SResult<(i32, i32)>;
}

// Apply reader to Vec
impl PacketReader for Vec<u8> {
    // Read a single byte as u8 ( 8-Bit Unsigned Integer )
    fn get_u8(&mut self) -> u8 {
        self.remove(0)
    }

    // Read a single byte as i8 ( 8-Bit Integer )
    fn get_i8(&mut self) -> i8 {
        self.remove(1) as i8
    }

    // Read a two bytes as u16 ( 16-Bit Unsigned Integer )
    fn get_u16(&mut self) -> u16 {
        u16::from_be_bytes([self.get_u8(), self.get_u8()])
    }

    // Read a two bytes as i16 ( 16-Bit Integer )
    fn get_i16(&mut self) -> i16 {
        i16::from_be_bytes([self.get_u8(), self.get_u8()])
    }

    // Read a VarInt ( Dynamic-length 32-Bit Integer )
    fn get_varint(&mut self) -> SResult<i32> {
        // Result variable
        let mut ans = 0;
        // Read up to 4 bytes
        for i in 0..4 {
            // Read one byte
            let buf = self.get_u8();
            // Calculate res with bit moving and another
            ans |= ((buf & 0b0111_1111) as i32) << 7 * i;
            // If it's limit when stop reading
            if buf & 0b1000_0000 == 0 {
                break;
            }
        }
        // Return result as successful
        Ok(ans)
    }

    // Read a Long ( 64-Bit Integer )
    fn get_i64(&mut self) -> i64 {
        // Yes, read 8 bytes
        i64::from_be_bytes([
            self.get_u8(),
            self.get_u8(),
            self.get_u8(),
            self.get_u8(),
            self.get_u8(),
            self.get_u8(),
            self.get_u8(),
            self.get_u8(),
        ])
    }

    // Read a String ( VarInt as len; bytes[::len] )
    fn get_string(&mut self) -> SResult<String> {
        // Getting string-length
        let len = self.get_varint()?;
        // Create String's bytes buffer
        let mut buf = Vec::new();
        // Reading Bytes
        for _ in 0..len {
            buf.push(self.get_u8())
        }
        // Convert Bytes to UTF8 String
        match String::from_utf8(buf) {
            Ok(v) => Ok(v),
            Err(_) => Err(SimpleError(String::from("Failed to parse chars"), None)),
        }
    }
    // Read first two VarInt(Packet's length and id)
    fn read_base(&mut self) -> SResult<(i32, i32)> {
        let len = self.get_varint()?;
        let pid = self.get_varint()?;
        Ok((len, pid))
    }
}
