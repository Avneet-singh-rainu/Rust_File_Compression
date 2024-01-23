extern crate flate2;

use flate2::{write::ZlibEncoder, *};
use std::{
    fs::File,
    io::{Read, Write},
};

fn main() {
    //open the file to be compressed...
    let mut input = File::open("book.pdf").unwrap();

    //created a targeted file or a new compressed file...
    let mut output = File::create("compressed.pdf").unwrap();

    //created a buffer  to hold the compressed data...
    let mut buffer = Vec::new();

    //read the original file into the buffer...
    input.read_to_end(&mut buffer).unwrap();

    //we are creating an encoder which will compress the data into the new buffer that we declared...
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

    //now we are writing the data into the encoder buffer to be compressed...
    encoder.write_all(&mut buffer).unwrap();

    //after all the data has been written to encoder buffer it will compress the data and after finishing the data will be stored in compressed_data
    let compressed_data = encoder.finish().unwrap();

    //we are writing the compressed data into new  file that we created above...
    output.write_all(&compressed_data).unwrap();
}
