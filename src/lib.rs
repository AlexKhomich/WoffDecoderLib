extern crate byteorder;
extern crate flate2;

pub mod utils;
mod structures;

use crate::structures::*;
use crate::utils::*;
use std::mem::size_of;
use flate2::{Decompress, FlushDecompress};
use std::ffi::CStr;
use std::os::raw::c_char;
use bytebuffer::ByteBuffer;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_to_buffer() {
        let str_path = "test_fonts/noto-sans-tc.woff";
        let mut buf: Vec<u8> = vec![];
        read_file(str_path, &mut buf);
        let result = DecodedResult::create_ok_result(decode_internal(&mut buf));
        match result {
            Ok(data) => { debug_assert!(data.len() > 0) }
            Err(err) => { debug_assert!(err != Error::None) }
        }
    }

    #[test]
    fn test_read_file() {
        let str_path = "test_fonts/noto-sans-tc.woff";
        let mut buf: Vec<u8> = vec![];
        let read_result = read_file(str_path, &mut buf);
        debug_assert!(
            !str_path.is_empty()
                && buf.len() > 0
                && read_result.error == Error::None
                && read_result.data_len > 0
        )
    }

    #[test]
    fn test_sanity_check() {
        let str_path = "test_fonts/noto-sans-tc.woff";
        let mut buf: Vec<u8> = vec![];
        read_file(str_path, &mut buf);
        debug_assert!(sanity_check(&mut buf) == Error::None)
    }
}

/// Result structure with decoded SFNT data
///
/// #Fields
///
/// `decoded_data` - decoded SFNT data
/// `decoded_data_len` - length of decoded SFNT data
/// `error` - type of error. None - returned result has no errors.
#[repr(C)]
pub struct DecodedResult {
    pub decoded_data: *mut u8,
    pub decoded_data_len: usize,
    pub error: Error,
}

struct DecodedData {
    pub sfnt_header: SfntOffsetTable,
    pub table_records: Vec<SfntTableRecord>,
    pub data_tables: Vec<Vec<u8>>,
    pub error: Error
}

/// Creates `DecodedResult` structure with null decoded data pointer,
/// zero decoded data length and error type fields
impl DecodedResult {

    fn create_error_result_ptr(err: Error) -> *mut Self {
        Box::into_raw(Box::new(Self {
            decoded_data: std::ptr::null_mut(),
            decoded_data_len: 0,
            error: err,
        }))
    }

    fn create_ok_result(result: Result<DecodedData, Error>) -> Result<Vec<u8>, Error> {
        match result {
            Ok(data) => {
                Ok(assemble_sfnt_data_vec(
                    data.sfnt_header,
                    data.table_records,
                    data.data_tables
                ))
            }
            Err(err) => { Err(err) }
        }
    }

    fn create_ok_result_ptr(result: Result<DecodedData, Error>) -> *mut Self {
        return match result {
            Ok(data) => {
                assemble_sfnt_binary(
                    data.sfnt_header,
                    data.table_records,
                    data.data_tables,
                    data.error
                )
            }
            Err(err) => {
                DecodedResult::create_error_result_ptr(err)
            }
        }
    }
}

/// `FileRWResult` structure with length of decoded data and error
///
/// #Fields
///
/// `data_len` - length of decoded SFNT data that was written to file.
/// `error` - type of error. None - returned result has no errors and file was written successfully.
#[repr(C)]
pub struct FileRWResult {
    pub data_len: usize,
    pub error: Error,
}

/// Creates `FileRWResult` structure with null decoded data pointer,
/// zero decoded data length and error type fields
impl FileRWResult {
    fn create_error_result_ptr(err: Error) -> *mut Self {
        Box::into_raw(Box::new(Self {
            data_len: 0,
            error: err,
        }))
    }

    fn create_ok_result(result: Result<DecodedData, Error>, out_path: &str) -> Error {
        match result {
            Ok(data) => {
                create_sfnt_file_from_vec(
                    data.sfnt_header,
                    data.table_records,
                    data.data_tables,
                    out_path
                )
            }
            Err(err) => { err }
        }
    }

    fn create_ok_result_ptr(result: Result<DecodedData, Error>, out_path: &str) -> *mut Self {
        return match result {
            Ok(data) => {
                create_sfnt_file(
                    data.sfnt_header,
                    data.table_records,
                    data.data_tables,
                    out_path
                )
            }
            Err(err) => {
                FileRWResult::create_error_result_ptr(err)
            }
        }
    }
}

/// Enum with types of error
/// If `Error` with type `None` that means no errors occurred
#[repr(C)]
#[derive(PartialEq, Debug)]
pub enum Error {
    None,
    DecodeError,
    DataSourceIsEmpty,
    OutBufferFull,
    BuffError,
    DecompressError,
    InvalidWoffSize,
    InputBufferIsEmpty,
    InvalidWoffSignature,
    InvalidWoffStructure,
    CreateFileError,
    OpenFileError,
    WriteToFileError,
    ReadFromFileError,
    InputPathError,
    OutputPathError,
}

/// Decode .woff file data to SFNT bytes wrapped for using with C wrapper
/// And returns Result structure with decoded data
#[no_mangle]
pub unsafe extern fn decode_from_file_wrapped(path: *const c_char) -> *mut DecodedResult {
    let c_srt = CStr::from_ptr(path);
    let str_path = match c_srt.to_str() {
        Ok(string) => string,
        Err(_) => return DecodedResult::create_error_result_ptr(Error::InputPathError)
    };
    let mut buf: Vec<u8> = vec![];
    let read_result = read_file(str_path, &mut buf);
    if read_result.error != Error::None {
        return DecodedResult::create_error_result_ptr(read_result.error);
    }
    DecodedResult::create_ok_result_ptr(decode_internal(&mut buf))
}

/// Decode WOFF data to SFNT data wrapped for using with C wrapper
#[no_mangle]
pub unsafe extern fn decode_from_data_wrapped(
    source_buf: *const u8,
    woff_data_size: usize,
) -> *mut DecodedResult {
    if !source_buf.is_null() && woff_data_size > 0 {
        let mut data: Vec<u8> = Vec::from_raw_parts(
            source_buf as *mut u8,
            woff_data_size,
            woff_data_size,
        );
        DecodedResult::create_ok_result_ptr(decode_internal(&mut data))
    } else {
        DecodedResult::create_error_result_ptr(Error::DecodeError)
    }
}

/// Decode .woff file data to SFNT file wrapped for using with C wrapper
/// And returns FileRWResult structure with decoded data
#[no_mangle]
pub unsafe extern fn decode_file_to_file_wrapped(
    in_path: *const c_char,
    out_path: *const c_char,
) -> *mut FileRWResult {
    let c_srt = CStr::from_ptr(in_path);
    let in_path = match c_srt.to_str() {
        Ok(string) => string,
        Err(_) => return FileRWResult::create_error_result_ptr(Error::InputPathError)
    };

    let c_srt = CStr::from_ptr(out_path);
    let out_path = match c_srt.to_str() {
        Ok(string) => string,
        Err(_) => return FileRWResult::create_error_result_ptr(Error::OutputPathError)
    };
    let mut buf: Vec<u8> = vec![];
    let read_result = read_file(in_path, &mut buf);
    if read_result.error != Error::None {
        return FileRWResult::create_error_result_ptr(read_result.error);
    }
    FileRWResult::create_ok_result_ptr(decode_internal(&mut buf), out_path)
}

/// Decode WOFF data to SFNT file wrapped for using with C wrapper
#[no_mangle]
pub unsafe extern fn decode_data_to_file_wrapped(
    source_buf: *const u8,
    woff_data_size: usize,
    path: *const c_char,
) -> *mut FileRWResult {
    let c_srt = CStr::from_ptr(path);
    let str_path = match c_srt.to_str() {
        Ok(string) => string,
        Err(_) => return FileRWResult::create_error_result_ptr(Error::InputPathError)
    };

    if !source_buf.is_null() && woff_data_size > 0 {
        let mut data: Vec<u8> = Vec::from_raw_parts(
            source_buf as *mut u8,
            woff_data_size,
            woff_data_size,
        );
        FileRWResult::create_ok_result_ptr(decode_internal(&mut data), str_path)
    } else {
        FileRWResult::create_error_result_ptr(Error::DecodeError)
    }
}

/// Destroys buffer with decoded data. Using with C wrapper
#[no_mangle]
pub unsafe extern fn destroy_decoded_result(data: *mut DecodedResult) {
    if !data.is_null() {
        if !(*data).decoded_data.is_null() {
            drop(Box::from_raw((*data).decoded_data));
        }
        drop(Box::from_raw(data));
    }
}

/// Destroys buffer with decoded data. Using with C wrapper
#[no_mangle]
pub unsafe extern fn destroy_file_rw_result(data: *mut FileRWResult) {
    if !data.is_null() {
        drop(Box::from_raw(data));
    }
}

/// Decode .woff file data to SFNT bytes. Using with C wrapper
pub fn decode_from_file(path: &str) -> Result<Vec<u8>, Error> {
    let mut buf: Vec<u8> = vec![];
    let read_result = read_file(path, &mut buf);
    if read_result.error != Error::None {
        return Err(read_result.error);
    };
    match DecodedResult::create_ok_result(decode_internal(&mut buf)) {
        Ok(result) => { Ok(result) }
        Err(err) => { Err(err) }
    }
}

/// Decode .woff file data to SFNT file. Using with C wrapper
pub fn decode_from_file_to_file(in_path: &str, out_path: &str) -> Error {
    let mut buf: Vec<u8> = vec![];
    let read_result = read_file(in_path, &mut buf);
    if read_result.error != Error::None {
        return read_result.error;
    }
   FileRWResult::create_ok_result(decode_internal(&mut buf), out_path)
}

/// Decode WOFF data from vector to SFNT data
pub fn decode_from_vec(buf: &mut Vec<u8>) -> Result<Vec<u8>, Error> {
    match DecodedResult::create_ok_result(decode_internal(buf)) {
        Ok(result) => { Ok(result) }
        Err(err) => { Err(err) }
    }
}

/// Decode WOFF data from vector to SFNT file
pub fn decode_from_vec_to_file(buf: &mut Vec<u8>, out_path: &str) -> Error {
    FileRWResult::create_ok_result(decode_internal(buf), out_path)
}

/// Decode WOFF data from slice to SFNT data
pub fn decode_from_slice(buf: &[u8]) -> Result<Vec<u8>, Error> {
    let mut data: Vec<u8> = Vec::from(buf);
    match DecodedResult::create_ok_result(decode_internal(&mut data)) {
        Ok(result) => { Ok(result) }
        Err(err) => { Err(err) }
    }
}

/// Decode WOFF data from slice to SFNT file
pub fn decode_from_slice_to_file(buf: &[u8], out_path: &str) -> Error {
    let mut data: Vec<u8> = Vec::from(buf);
    FileRWResult::create_ok_result(decode_internal(&mut data), out_path)
}

/// Sanity check for WOFF file
fn sanity_check(buf: &mut Vec<u8>) -> Error {
    if buf.is_empty() { return Error::InputBufferIsEmpty; }
    if buf.len() < size_of::<WoffHeader>() { return Error::InvalidWoffSize; }

    let mut woff_signature_buf = ByteBuffer::from_bytes(&[b'w', b'O', b'F', b'F']);
    let woff_signature = read_u32_be(&mut woff_signature_buf);
    let woff_header = create_woff_header(buf);

    if woff_header.signature != woff_signature { return Error::InvalidWoffSignature; }
    if woff_header.length != buf.len() as u32 { return Error::InvalidWoffSize; }

    let sfnt_num_tables = woff_header.num_tables;
    if buf.len() < (size_of::<WoffHeader>()
        + sfnt_num_tables as usize * size_of::<WoffTableDirectoryEntry>()) {
        return Error::InvalidWoffSize;
    }

    // we do not check other things 'cause it may lead to performance issues
    // we can return error a little bit later if SFNT tables have corrupted data
    Error::None
}

/// Main function to decode and construct SFNT file or data form WOFF file
fn decode_internal(mut buf: &mut Vec<u8>) -> std::result::Result<DecodedData, Error> {
    let mut error = sanity_check(buf);

    // return result with error from sanity check if error occurred
    if error != Error::None {
        return Err(error);
    }

    // We need to know sizes of several SFNT and WOFF structures.
    let sfnt_offset_table_size = size_of::<SfntOffsetTable>();
    let sfnt_table_record_size = size_of::<SfntTableRecord>();
    let woff_table_directory_size = size_of::<WoffTableDirectoryEntry>();
    let woff_header_size = size_of::<WoffHeader>();

    // Construct WOFF header.
    let woff_header = create_woff_header(buf);

    let search_range = calculate_search_range(woff_header.num_tables);
    let entry_selector = calculate_entry_selector(search_range);
    let range_shift = calculate_range_shift(woff_header.num_tables, search_range);

    // Construct SFNT header (sfnt_offset_table) with its builder.
    let sfnt_offset_table = SfntOffsetTable {
        version: woff_header.flavor,
        num_tables: woff_header.num_tables,
        search_range,
        entry_selector,
        range_shift,
    };

    let sfnt_num_tables = sfnt_offset_table.num_tables;
    let mut sfnt_table_offset = sfnt_offset_table_size;

    let mut woff_table_dir_entry_container: Vec<WoffTableDirectoryEntry> = Vec::with_capacity(sfnt_num_tables as usize);

    // construct each SFNT table record
    for i in 0..sfnt_num_tables as usize {
        let next_table_offset = woff_header_size + (i * woff_table_directory_size);
        let woff_table_dir_entry = create_woff_table_dir_entry(&mut buf, next_table_offset);
        // check if dir_entry parameters are correct
        // and if not return Result with error
        if (woff_table_dir_entry.orig_length < woff_table_dir_entry.comp_length)
            || (woff_table_dir_entry.offset as usize > buf.len() - woff_table_dir_entry.comp_length as usize) {
            error = Error::InvalidWoffStructure;
            return Err(error);
        }
        woff_table_dir_entry_container.push(woff_table_dir_entry);
        sfnt_table_offset += sfnt_table_record_size
    }

    // sort all entries by tag
    woff_table_dir_entry_container.sort_by(
        |a, b| a.tag.cmp(&b.tag)
    );

    let mut sfnt_table_records_vec: Vec<SfntTableRecord> = Vec::with_capacity(sfnt_num_tables as usize);
    let mut sfnt_table_data_vec: Vec<Vec<u8>> = Vec::with_capacity(sfnt_num_tables as usize);

    for table_dir_entry in woff_table_dir_entry_container.iter().take(sfnt_num_tables as usize) {
        let start_offset = table_dir_entry.offset as usize;
        let end_offset = (table_dir_entry.offset + table_dir_entry.comp_length) as usize;

        let mut sfnt_table_data: Vec<u8> = Vec::with_capacity(table_dir_entry.orig_length as usize);
        let source_slice = &buf[start_offset..end_offset];

        if table_dir_entry.orig_length != table_dir_entry.comp_length {
            // decompress table data
            let mut decompressor = Decompress::new(true);
            // match decompressor result status
            match decompressor.decompress_vec(source_slice, &mut sfnt_table_data, FlushDecompress::None) {
                Ok(stat) => {
                    if stat == flate2::Status::Ok {
                        error = Error::OutBufferFull;
                        return Err(error);
                    };
                    if stat == flate2::Status::BufError {
                        error = Error::BuffError;
                        return Err(error);
                    };
                    if stat == flate2::Status::StreamEnd {
                        error = Error::None
                    };
                }
                Err(_) => {
                    error = Error::DecompressError;
                    return Err(error);
                }
            };
        } else {
            sfnt_table_data.extend_from_slice(source_slice);
        }

        let snft_table_record = SfntTableRecord {
            table_tag: table_dir_entry.tag,
            checksum: table_dir_entry.orig_checksum,
            offset: sfnt_table_offset as u32,
            length: table_dir_entry.orig_length,
        };

        sfnt_table_records_vec.push(snft_table_record);

        // needs to check table record len on 4-bytes alignment and if it's not - add zero-bytes to each unalignment table
        if table_dir_entry.orig_length % 4 != 0 {
            let padded_len = calculate_padded_len(table_dir_entry.orig_length, sfnt_table_data.len());

            for _ in 0..padded_len {
                sfnt_table_data.push(b'\0');
            }

            sfnt_table_offset += sfnt_table_data.len()
        } else {
            sfnt_table_offset += table_dir_entry.orig_length as usize
        }

        sfnt_table_data_vec.push(sfnt_table_data);
    }

    Ok(DecodedData {
        sfnt_header: sfnt_offset_table,
        table_records: sfnt_table_records_vec,
        data_tables: sfnt_table_data_vec,
        error
    })
}

/// Function for creating WOFF header from raw data
fn create_woff_header(buf: &mut Vec<u8>) -> WoffHeader {
    let mut buffer = ByteBuffer::from_bytes(buf);
    WoffHeader {
        signature: read_u32_be(&mut buffer),
        flavor: read_u32_be(&mut buffer),
        length: read_u32_be(&mut buffer),
        num_tables: read_u16_be(&mut buffer),
        reserved: read_u16_be(&mut buffer),
        total_sfnt_size: read_u32_be(&mut buffer),
        major_version: read_u16_be(&mut buffer),
        minor_version: read_u16_be(&mut buffer),
        meta_offset: read_u32_be(&mut buffer),
        meta_length: read_u32_be(&mut buffer),
        meta_orig_length: read_u32_be(&mut buffer),
        priv_offset: read_u32_be(&mut buffer),
        priv_length: read_u32_be(&mut buffer),
    }
}

/// function for creating WOFF table directory entry structure
fn create_woff_table_dir_entry(buf: &mut Vec<u8>, next_table_offset: usize) -> WoffTableDirectoryEntry {
    let mut buffer = ByteBuffer::from_bytes(buf);
    buffer.set_rpos(next_table_offset);
    WoffTableDirectoryEntry {
        tag: read_u32_be(&mut buffer),
        offset: read_u32_be(&mut buffer),
        comp_length: read_u32_be(&mut buffer),
        orig_length: read_u32_be(&mut buffer),
        orig_checksum: read_u32_be(&mut buffer),
    }
}

/// Creates SFNT binary from parts of data and returns raw pointer on this data
/// Important note: vector with decoded data doesn't destruct after finishing function!
/// After calling wrapped functions you should call destroy function and set result data as a parameter
fn assemble_sfnt_binary(
    sfnt_header: SfntOffsetTable,
    table_records: Vec<SfntTableRecord>,
    data_tables: Vec<Vec<u8>>,
    error: Error,
) -> *mut DecodedResult {
    let mut sfnt_header_data = sfnt_header.transform_to_u8_vec();
    let mut sfnt_data_vec: Vec<u8> = Vec::with_capacity(
        sfnt_header_data.len()
            + table_records.len()
            + data_tables.len()
    );

    sfnt_data_vec.append(&mut sfnt_header_data);

    for record in table_records {
        let mut record_data = record.transform_to_u8_vec();
        sfnt_data_vec.append(&mut record_data);
    }

    for mut table in data_tables {
        sfnt_data_vec.append(&mut table)
    }

    let data_len = sfnt_data_vec.len();
    let data = sfnt_data_vec.as_mut_ptr();
    std::mem::forget(sfnt_data_vec);
    let result_buffer = DecodedResult {
        decoded_data: data,
        decoded_data_len: data_len,
        error,
    };
    Box::into_raw(Box::new(result_buffer))
}

/// Creates SFNT binary from parts of data and returns Vec<u8> with data
fn assemble_sfnt_data_vec(
    sfnt_header: SfntOffsetTable,
    table_records: Vec<SfntTableRecord>,
    data_tables: Vec<Vec<u8>>
) -> Vec<u8> {
    let mut sfnt_header_data = sfnt_header.transform_to_u8_vec();
    let mut sfnt_data_vec: Vec<u8> = Vec::with_capacity(
        sfnt_header_data.len()
            + table_records.len()
            + data_tables.len()
    );

    sfnt_data_vec.append(&mut sfnt_header_data);

    for record in table_records {
        let mut record_data = record.transform_to_u8_vec();
        sfnt_data_vec.append(&mut record_data);
    }

    for mut table in data_tables {
        sfnt_data_vec.append(&mut table)
    }
    return sfnt_data_vec
}

/// Creates SFNT binary from parts of data and call function for creating .ttf file
fn create_sfnt_file(
    sfnt_header: SfntOffsetTable,
    table_records: Vec<SfntTableRecord>,
    data_tables: Vec<Vec<u8>>,
    path_to_out_file: &str,
) -> *mut FileRWResult {
    let mut sfnt_header_data = sfnt_header.transform_to_u8_vec();
    let mut sfnt_data_vec: Vec<u8> = Vec::with_capacity(
        sfnt_header_data.len()
            + table_records.len()
            + data_tables.len()
    );

    sfnt_data_vec.append(&mut sfnt_header_data);

    for record in table_records {
        let mut record_data = record.transform_to_u8_vec();
        sfnt_data_vec.append(&mut record_data);
    }

    for mut table in data_tables {
        sfnt_data_vec.append(&mut table)
    }

    Box::into_raw(Box::new(create_ttf_file(&sfnt_data_vec.as_slice(), path_to_out_file)))
}

/// Creates SFNT binary from parts of data and call function for creating .ttf file
fn create_sfnt_file_from_vec(
    sfnt_header: SfntOffsetTable,
    table_records: Vec<SfntTableRecord>,
    data_tables: Vec<Vec<u8>>,
    path_to_out_file: &str,
) -> Error {
    let mut sfnt_header_data = sfnt_header.transform_to_u8_vec();
    let mut sfnt_data_vec: Vec<u8> = Vec::with_capacity(
        sfnt_header_data.len()
            + table_records.len()
            + data_tables.len()
    );

    sfnt_data_vec.append(&mut sfnt_header_data);

    for record in table_records {
        let mut record_data = record.transform_to_u8_vec();
        sfnt_data_vec.append(&mut record_data);
    }

    for mut table in data_tables {
        sfnt_data_vec.append(&mut table)
    }

    create_ttf_file(&sfnt_data_vec.as_slice(), path_to_out_file).error
}