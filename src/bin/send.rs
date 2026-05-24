use qrky::AckPacket;
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
}