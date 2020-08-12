//! This is the sample test implementation of the library functionality

mod utils;

use woffdecoder::{Error, FileRWResult, decode_from_vec_to_file, decode_from_vec};
use crate::utils::read_file;

fn handle_error(err: Error) {
    match err {
        Error::None => { println!("No errors") }
        Error::DecodeError => { println!("Decode error") }
        Error::DataSourceIsEmpty => { println!("Data source is empty") }
        Error::OutBufferFull => { println!("Out buffer is full") }
        Error::BuffError => { println!("Buffer error") }
        Error::DecompressError => { println!("Decompress error") }
        Error::InvalidWoffSize => { println!("Invalid WOFF size") }
        Error::InputBufferIsEmpty => { println!("Input buffer is empty") }
        Error::InvalidWoffSignature => { println!("Invalid WOFF signature") }
        Error::InvalidWoffStructure => { println!("Invalid WOFF structure") }
        Error::CreateFileError => { println!("Create file error") }
        Error::OpenFileError => { println!("Open file error") }
        Error::WriteToFileError => { println!("Write to file error") }
        Error::ReadFromFileError => { println!("Read from file error") }
        Error::InputPathError => { println!("Input file path error") }
        Error::OutputPathError => { println!("Output file path error") }
    }
}

fn decode_to_file() {
    let mut buf: Vec<u8> = vec![];
    let in_path = "test_fonts/noto-sans-tc.woff"; /*change to your path*/
    let out_path = "test_fonts/noto-sans-tc.ttf"; /*your out file name*/
    let rw_result: FileRWResult = read_file(in_path, &mut buf);
    if rw_result.error == Error::None {
        let result = decode_from_vec_to_file(&mut buf, out_path);
        if result != Error::None {
            handle_error(result);
        } else {
            println!("The WOFF data was successfully decoded to TTF font data and written to file: {}", out_path);
        }
    }
}

fn decode_to_buffer() {
    let mut buf: Vec<u8> = vec![];
    let path = "test_fonts/noto-sans-tc.woff"; /*change to your path*/
    let rw_result: FileRWResult = read_file(path, &mut buf);
    if rw_result.error == Error::None {
        let result = decode_from_vec(&mut buf);
        match result {
            Ok(result_vec) => {
                println!("The WOFF data was successfully decoded to TTF font data with {} bytes size!", result_vec.len());
            }
            Err(err) => {
                handle_error(err);
            }
        }
    }
}

fn main() {
    decode_to_file();
    decode_to_buffer();
}