mod mca_reader;
use std::env::{args, Args};
use std::io::Cursor;

fn main() {
    let mut args: Args = args();
    let file_path: String = args.nth(1).unwrap();

    let file_bytes = std::fs::read(file_path).unwrap();
    let mut cursor = Cursor::new(file_bytes);

    mca_reader::read_mca(&mut cursor);

    //println!("{:?}", file_bytes);
}
