use std::fs::File;
use std::io::{Read, Write};
use flate2::write::GzEncoder;
use flate2::Compression;

fn main() {
   
    let mut input_file = File::open("/app/large_file.txt").expect("Failed to open file");

    // Create the output compressed file in the bind-mounted directory
    let output_file = File::create("/app/output_file.gz").expect("Failed to create output file");

    // Create a gzip encoder
    let mut encoder = GzEncoder::new(output_file, Compression::default());

    // Read the input file and compress it
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer).expect("Failed to read input file");
    encoder.write_all(&buffer).expect("Failed to write compressed data");

    encoder.finish().expect("Failed to finish compression");
    println!("Compression complete, output saved to /app/output_file.gz");
}

