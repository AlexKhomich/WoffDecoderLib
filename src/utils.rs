use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use bytebuffer::ByteBuffer;


/// Reads data from file to buffer
/// If error occurs - prints path to file and err description to stdout
pub fn read_file(path: &str, buf: &mut Vec<u8>) -> crate::FileRWResult {
    let mut file_size: usize = 0;
    let mut error = crate::Error::None;
    match File::open(path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            match reader.read_to_end(buf) {
                Ok(result_size) => { file_size = result_size },
                Err(err) => {
                    println!("Couldn't read the file: {}, cause: {}", path, err.to_string());
                    error = crate::Error::ReadFromFileError;
                },
            }
        },
        Err(err) => {
            println!("Couldn't open the file: {}, cause: {}", path, err.to_string());
            error = crate::Error::OpenFileError
        },
    }
    crate::FileRWResult {
        data_len: file_size,
        error
    }
}

/// Creates .ttf file and writes all decoded data to this file
/// If error occurs - prints path to file and err description to stdout
#[allow(dead_code)]
pub(crate) fn create_ttf_file(data_slice: &[u8], path_to_out_file: &str) -> crate::FileRWResult {
    let mut error = crate::Error::None;
    match File::create(path_to_out_file) {
        Ok(mut file) => {
            let data_slice = data_slice;
            match file.write_all(data_slice) {
                Ok(_) => {},
                Err(err) => {
                    println!("Couldn't write to file: {}, cause: {}", path_to_out_file, err.to_string());
                    error = crate::Error::WriteToFileError;
                },
            };
        },
        Err(err) => {
            println!("Couldn't create the file: {}, cause: {}", path_to_out_file, err.to_string());
            error = crate::Error::CreateFileError;
        },
    };
    crate::FileRWResult {
        data_len: data_slice.len(),
        error
    }
}

/// This one reads unsigned 32-bits value in big endian order
/// If error occurs - panic with message
#[allow(dead_code)]
pub(crate) fn read_u32_be(buf: &mut ByteBuffer) -> u32 {
    let part = buf.read_bytes(std::mem::size_of::<u32>());
    let mut rdr = Cursor::new(part);
    rdr.read_u32::<BigEndian>().expect("Error: couldn't read u32 value from buffer")
}

/// This one reads unsigned 16-bits value in big endian order
/// If error occurs - panic with message
#[allow(dead_code)]
pub(crate) fn read_u16_be(buf: &mut ByteBuffer) -> u16 {
    let part = buf.read_bytes(std::mem::size_of::<u16>());
    let mut rdr = Cursor::new(part);
    rdr.read_u16::<BigEndian>().expect("Error: couldn't read u16 value from buffer")
}

/// Calculates the entrySelector that is log2(maximum power of 2 <= numTables).
/// It tells how many iterations of the search loop are needed.
/// (i.e. how many times to cut the range in half)
#[allow(dead_code)]
#[inline(always)]
pub(crate) fn calculate_entry_selector(mut number: u16) -> u16 {
    let mut res: u16 = 0;
    while number > 16 {
        number >>= 1;
        res += 1;
    }
    res
}

/// Calculates rangeShift (numTables*16-searchRange)
#[allow(dead_code)]
#[inline(always)]
pub(crate) fn calculate_range_shift(num_tables: u16, search_range: u16) -> u16 {
    num_tables * 16 - search_range
}

/// Calculates search range for every SFNT data table.
/// This one has to be (maximum power of 2 <= numTables)*16.
///     For example:
///     result = Math.pow(2, Math.floor(Math.log(num_tables) / Math.log(2)));
///     result * 16;
///     For range [1; 2) returned value will be 16; [2; 4) -> 32; [4; 8) -> 64; [8; 16) -> 128 etc.
#[allow(dead_code)]
#[inline(always)]
pub(crate) fn calculate_search_range(num_tables: u16) -> u16 {
    let mut sr = num_tables;
    sr |= sr >> 1;
    sr |= sr >> 2;
    sr |= sr >> 4;
    sr |= sr >> 8;
    sr &= !(sr >> 1);
    sr *= 16;
    sr
}

/// Calculates padded length for structure that has to be aligned by 4-bytes.
#[allow(dead_code)]
#[inline(always)]
pub(crate) fn calculate_padded_len(orig_len: u32, sfnt_table_data_len: usize) -> u32 {
    let aligned_len = (orig_len + 3) & !3;
    aligned_len - sfnt_table_data_len as u32
}

/// Works only with the little endian order.
/// Result slice will be in the little endian order!
/// DO NOT USE it with Big endian order!!!
#[allow(dead_code)]
#[inline(always)]
pub(crate) unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        std::mem::size_of::<T>(),
    )
}

/// Transforms unsigned 32-bits number to array of bytes.
/// Result array contains values in big endian order!
#[allow(dead_code)]
#[inline(always)]
pub(crate) fn u32_to_u8_array(x: u32) -> [u8; 4] {
    let mut result: [u8; 4] = [0; 4];
    result[0] = ((x >> 24) & 0xff) as u8;
    result[1] = ((x >> 16) & 0xff) as u8;
    result[2] = ((x >> 8) & 0xff) as u8;
    result[3] = (x & 0xff) as u8;
    result
}

/// Transforms unsigned 16-bits number to array of bytes.
/// Result array contains values in big endian order!
#[allow(dead_code)]
#[inline(always)]
pub(crate) fn u16_to_u8_array(x: u16) -> [u8; 2] {
    let mut result: [u8; 2] = [0; 2];
    result[0] = ((x >> 8) & 0xff) as u8;
    result[1] = (x & 0xff) as u8;
    result
}

/// Transforms unsigned 32-bits number to vector of bytes.
/// Result vector contains values in big endian order!
#[allow(dead_code)]
#[inline(always)]
pub(crate) fn transform_u32_to_u8_vec(x: u32) -> Vec<u8> {
    let result: [u8; 4] = x.to_be_bytes();
    result.to_vec()
}

/// Transforms unsigned 16-bits number to vector of bytes.
/// Result vector contains values in big endian order!
#[allow(dead_code)]
#[inline(always)]
pub(crate) fn transform_u16_to_u8_vec(x: u16) -> Vec<u8> {
    let mut result_vec: Vec<u8> = Vec::with_capacity(2);
    result_vec.push(((x >> 8) & 0xff) as u8);
    result_vec.push((x & 0xff) as u8);
    result_vec
}