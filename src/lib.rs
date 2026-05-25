use fast_qr::{QRBuilder, QRCode, ECL, Version, Mode};
use nokhwa::Camera;
use nokhwa::pixel_format::LumaFormat;
use rqrr::PreparedImage;

pub const CRC: crc::Crc<u64> = crc::Crc::<u64>::new(&crc::CRC_64_XZ);

pub struct SendPacket {
    retry_count: u64,
    packet_number: u64,
    checksum: u64,
    data: [u8; 2048],
}

pub struct AckPacket {
    packet_number: u64,
    checksum: u64
}

impl SendPacket {
    pub fn from_bytes(data: [u8; 2072]) -> Self {
        SendPacket {
            retry_count: u64::from_be_bytes(data[0..8].try_into().unwrap()),
            packet_number: u64::from_be_bytes(data[8..16].try_into().unwrap()),
            checksum: u64::from_be_bytes(data[16..24].try_into().unwrap()),
            data: data[24..].try_into().unwrap()
        }
    }

    fn to_bytes(&self) -> [u8; 2072] {
        let mut data = [0; 2072];
        u64_to_be_bytes(&mut data[0..8], self.retry_count);
        u64_to_be_bytes(&mut data[8..16], self.packet_number);
        u64_to_be_bytes(&mut data[16..24], self.checksum);
        data[24..].copy_from_slice(&self.data);

        data
    }

    pub fn generate_qr(&self) -> QRCode {
        QRBuilder::new(self.to_bytes())
            .mode(Mode::Byte)
            .version(Version::V38)
            .ecl(ECL::M)
            .build().unwrap()
    }

    pub fn retry_count(&self) -> u64 { self.retry_count }
    pub fn packet_number(&self) -> u64 { self.packet_number }
    pub fn checksum(&self) -> u64 { self.checksum }
    pub fn to_data(self) -> [u8; 2048] { self.data }
}

impl AckPacket {
    pub fn from_bytes(data: &[u8; 16]) -> Self {
        AckPacket {
            packet_number: u64::from_be_bytes(data[0..8].try_into().unwrap()),
            checksum: u64::from_be_bytes(data[8..16].try_into().unwrap())
        }
    }

    pub fn from_send_packet(packet: &SendPacket) -> Self {
        AckPacket {
            packet_number: packet.packet_number,
            checksum: CRC.checksum(&packet.data)
        }
    }

    pub fn to_bytes(&self) -> [u8; 16] {
        let mut data = [0; 16];
        u64_to_be_bytes(&mut data[0..8], self.packet_number);
        u64_to_be_bytes(&mut data[8..16], self.checksum);

        data
    }

    pub fn generate_qr(&self) -> QRCode {
        QRBuilder::new(self.to_bytes())
            .mode(Mode::Byte)
            .version(Version::V02)
            .ecl(ECL::Q)
            .build().unwrap()
    }

    pub fn packet_number(&self) -> u64 { self.packet_number }
    pub fn checksum(&self) -> u64 { self.checksum }
}

/// Split up the unsigned 64-bit num and place it into the u8 buf (big endian).
fn u64_to_be_bytes(buf: &mut [u8], num: u64) {
    buf[0] = (num >> 56) as u8;
    buf[1] = (num >> 48) as u8;
    buf[2] = (num >> 40) as u8;
    buf[3] = (num >> 32) as u8;
    buf[4] = (num >> 24) as u8;
    buf[5] = (num >> 16) as u8;
    buf[6] = (num >> 8) as u8;
    buf[7] = num as u8;
}

/// Given the bytes of an assumed numeric string, convert it to an integer.
pub fn numeric_string_bytes_to_int(bytes: &[u8]) -> u64 {
    let mut num = 0;
    for &b in bytes {
        let digit = (b as char).to_digit(10).unwrap() as u64;
        num = num * 10 + digit;
    }
    num
}

/// Use the desired camera to scan for a QR code, outputting the data in the first one found.
pub fn scan_qr(camera: &mut Camera, expected_bytes: usize) -> Vec<u8> {
    loop {
        let frame = camera.frame().unwrap();
        let decoded = frame.decode_image::<LumaFormat>().unwrap();
        let mut prepared = PreparedImage::prepare(decoded);
        let qr_codes = prepared.detect_grids();
        if !qr_codes.is_empty() {
            let mut buf = Vec::with_capacity(expected_bytes);
            if qr_codes[0].decode_to(&mut buf).is_ok() {
                return buf;
            }
        }
    }
}