struct SendPacket {
    retry: bool,
    total_length: u64,
    packet_number: u64,
    checksum: u64,
    data: [u8; 2048],
}

struct RecvPacket {
    packet_number: u64,
    checksum: u64
}

impl SendPacket {
    fn to_bytes(&self) -> [u8; 2073] {
        let mut data = [0; 2073];
        data[0] = self.retry as u8;
        u64_to_bytes(&mut data[1..9], self.total_length);
        u64_to_bytes(&mut data[9..17], self.packet_number);
        u64_to_bytes(&mut data[17..25], self.checksum);
        data[25..].copy_from_slice(&self.data);

        data
    }
}

impl RecvPacket {
    fn to_bytes(&self) -> [u8; 16] {
        let mut data = [0; 16];
        u64_to_bytes(&mut data, self.packet_number);
        u64_to_bytes(&mut data, self.checksum);

        data
    }

    fn from_bytes(data: &[u8; 2073]) -> Self {
        RecvPacket {
            packet_number: u64::from_be_bytes(data[9..17].try_into().unwrap()),
            checksum: 0 // TODO: Real checksum
        }
    }
}

/// Split up the unsigned 64-bit num and place it into the u8 buf (big endian).
fn u64_to_bytes(buf: &mut [u8], num: u64) {
    buf[0] = (num >> 56) as u8;
    buf[1] = (num >> 48) as u8;
    buf[2] = (num >> 40) as u8;
    buf[3] = (num >> 32) as u8;
    buf[4] = (num >> 24) as u8;
    buf[5] = (num >> 16) as u8;
    buf[6] = (num >> 8) as u8;
    buf[7] = num as u8;
}

fn main() {
    println!("Hello, world!");
}
