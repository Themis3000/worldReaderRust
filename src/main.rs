mod mca_reader;
use std::env::{args, Args};
use std::io::Cursor;
use std::time::Instant;

fn main() {
    let mut args: Args = args();
    let file_path: String = args.nth(1).unwrap();

    let file_bytes = std::fs::read(file_path).unwrap();
    let mut cursor = Cursor::new(file_bytes);

    let now = Instant::now();

    let data = mca_reader::read_mca(&mut cursor);

    let elapsed = now.elapsed();
    println!("{:?}", data[0]);
    println!("Time taken: {:?}", elapsed);
}
