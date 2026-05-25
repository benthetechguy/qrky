use qrky::{numeric_string_bytes_to_int, scan_qr, AckPacket};
use qrky::SendPacket;
use std::process::exit;
use std::fs::File;
use fast_qr::{Mode, QRBuilder, Version, ECL};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = if args.len() > 1 {
        match File::open(&args[1]) {
            Ok(file) => {
                file
            },
            Err(error) => {
                eprintln!("Failed to read file {}: {}", args[1], error);
                exit(1);
            }
        }
    } else {
        eprintln!("Usage: {} <file>", args[0]);
        exit(2);
    };

    let len = file.metadata().unwrap().len();
    let qr = QRBuilder::new(len.to_string())
        .mode(Mode::Numeric)
        .version(Version::V01)
        .ecl(ECL::H)
        .build().unwrap();
    qr.print();

    let mut camera = nokhwa::Camera::new(nokhwa::utils::CameraIndex::Index(0), nokhwa::utils::RequestedFormat::new::<nokhwa::pixel_format::LumaFormat>(nokhwa::utils::RequestedFormatType::AbsoluteHighestFrameRate)).unwrap();
    camera.open_stream().unwrap();
    // Keep scanning QRs until len matches
    loop {
        let data = scan_qr(&mut camera, 7);
        let response_len = numeric_string_bytes_to_int(&data);
        if response_len == len {
            break;
        }
        println!("QR scanned, but wrong len")
    }
    println!("{}", len);
}