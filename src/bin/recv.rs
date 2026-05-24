use qrky::AckPacket;
use qrky::SendPacket;
use std::io::{stdout, Write};
use std::fs::File;
use std::process::exit;
use fast_qr::{Mode, QRBuilder, Version, ECL};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut output: Box<dyn Write> = if args.len() > 1 {
        if args[1] == "-" {
            Box::new(stdout())
        } else {
            match File::create(&args[1]) {
                Ok(file) => {
                    Box::new(file)
                },
                Err(err) => {
                    eprintln!("Failed to create file {}: {}", args[1], err);
                    exit(1);
                }
            }
        }
    } else {
        eprintln!("Usage: {} <output filename>", args[0]);
        exit(2);
    };
}