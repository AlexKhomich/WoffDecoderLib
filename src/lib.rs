extern crate byteorder;
extern crate flate2;

mod utils;
mod structures;

use crate::structures::*;
use crate::utils::*;

use std::mem::size_of;
use flate2::{Decompress, FlushDecompress};

use std::ffi::CStr;
use std::os::raw::c_char;

/// Decode .woff file data to SFNT bytes wrapped
#[no_mangle]
pub extern fn decode_from_file_wrapped(path: *const c_char, mut dest_buf: *mut u8) -> usize {
    let c_srt = unsafe {CStr::from_ptr(path)};
    let str_path = match c_srt.to_str() {
        Ok(string) => string,
        Err(_) => "error"
    };
    let mut buf: Vec<u8> = vec![];
    read_file(str_path, &mut buf);
    decode_internal(&mut buf, dest_buf)
}

/// Decode .woff file data to SFNT bytes
pub extern fn decode_from_file(path: &str, mut dest_buf: *mut u8) -> usize {
    let mut buf: Vec<u8> = vec![];
    read_file(path, &mut buf);
    decode_internal(&mut buf, dest_buf)
}

/// Decode WOFF data to SFNT data
#[no_mangle]
pub extern fn decode_from_data(source_buf: *const u8, woff_data_size: usize, dest_buf: *mut u8) -> usize {
    unimplemented!()
}

/// Function for creating WOFF header from raw data
fn create_woff_header(buf: &mut Vec<u8>) -> WoffHeader {
    WoffHeader {
        signature: read_u32_be(buf, Box::new(WoffHeaderRange::get_signature_range())),
        flavor: read_u32_be(buf, Box::new(WoffHeaderRange::get_flavor_range())),
        length: read_u32_be(buf, Box::new(WoffHeaderRange::get_length_range())),
        num_tables: read_u16_be(buf, Box::new(WoffHeaderRange::get_num_tables_range())),
        reserved: 0,
        total_sfnt_size: read_u32_be(buf, Box::new(WoffHeaderRange::get_total_sfnt_size_range())),
        major_version: read_u16_be(buf, Box::new(WoffHeaderRange::get_major_version_range())),
        minor_version: read_u16_be(buf, Box::new(WoffHeaderRange::get_minor_version_range())),
        meta_offset: read_u32_be(buf, Box::new(WoffHeaderRange::get_meta_offset_range())),
        meta_length: read_u32_be(buf, Box::new(WoffHeaderRange::get_meta_length_range())),
        meta_orig_length: read_u32_be(buf, Box::new(WoffHeaderRange::get_meta_orgig_length_range())),
        priv_offset: read_u32_be(buf, Box::new(WoffHeaderRange::get_priv_offset_range())),
        priv_length: read_u32_be(buf, Box::new(WoffHeaderRange::get_priv_length_range()))
    }
}

/// function for creating WOFF table directory entry structure
fn create_woff_table_dir_entry(buf: &mut Vec<u8>, next_table_offset: usize) -> WoffTableDirectoryEntry {
    WoffTableDirectoryEntry {
        tag: read_u32_be(buf, Box::new(
            WoffTableDirectoryEntryRange::construct_tag_range(next_table_offset, 4)
        )),
        offset: read_u32_be(buf, Box::new(
            WoffTableDirectoryEntryRange::construct_offset_range(next_table_offset, 4)
        )),
        comp_length: read_u32_be(buf, Box::new(
            WoffTableDirectoryEntryRange::construct_comp_length(next_table_offset, 4)
        )),
        orig_length: read_u32_be(buf, Box::new(
            WoffTableDirectoryEntryRange::construct_orig_length(next_table_offset, 4)
        )),
        orig_checksum: read_u32_be(buf, Box::new(
            WoffTableDirectoryEntryRange::construct_orig_checksum(next_table_offset, 4)
        ))
    }
}

/// Creates SFNT binary from parts of data and returns raw pointer on this data
fn assemble_sfnt_binary(
    sfnt_header: SfntOffsetTable,
    table_records: Vec<SfntTableRecord>,
    data_tables: Vec<Vec<u8>>,
    mut decoded_data: *mut u8
) -> usize {
    let mut sfnt_data_vec: Vec<u8> = vec![];
    let mut sfnt_header_data = sfnt_header.transform_to_u8_vec();
    sfnt_data_vec.append(&mut sfnt_header_data);

    for record in table_records {
        let mut record_data = record.transform_to_u8_vec();
        sfnt_data_vec.append(&mut record_data);
    };

    for mut table in data_tables {
        sfnt_data_vec.append(&mut table)
    }
    let decoded_data_len = sfnt_data_vec.len();
    decoded_data = sfnt_data_vec.as_mut_ptr();
    decoded_data_len
}

/// Main function to decode and construct SFNT file or data form WOFF file
fn decode_internal(mut buf: &mut Vec<u8>, mut decoded_data: *mut u8) -> usize {
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
        search_range: search_range,
        entry_selector: entry_selector,
        range_shift: range_shift
    };

    let sfnt_table_size = sfnt_offset_table.num_tables;
    let mut sfnt_table_offset = sfnt_offset_table_size;

    let mut woff_table_dir_entry_container: Vec<WoffTableDirectoryEntry> = vec![];

    // construct each SFNT table record
    for i in 0..sfnt_table_size as usize {
        let next_table_offset = woff_header_size + (i * woff_table_directory_size);
        let woff_table_dir_entry = create_woff_table_dir_entry(&mut buf, next_table_offset);
        woff_table_dir_entry_container.push(woff_table_dir_entry);
        sfnt_table_offset += sfnt_table_record_size
    }

    // sort all entries by tag
    woff_table_dir_entry_container.sort_by(
        |a, b| a.tag.cmp(&b.tag)
    );

    let mut sfnt_table_records_vec: Vec<SfntTableRecord> = vec![];
    let mut sfnt_table_data_vec: Vec<Vec<u8>> = vec![];

    for i in 0..sfnt_table_size as usize {
        let table_dir_entry = &woff_table_dir_entry_container[i];
        let start_offset = table_dir_entry.offset as usize;
        let end_offset = (table_dir_entry.offset + table_dir_entry.comp_length) as usize;

        let mut sfnt_table_data: Vec<u8> = Vec::with_capacity(table_dir_entry.orig_length as usize);
        let source_slice = &buf[start_offset..end_offset];

        if table_dir_entry.orig_length != table_dir_entry.comp_length {
            // decompress table data
            let mut decompressor = Decompress::new(true);
            let status = decompressor.decompress_vec(
                source_slice,
                &mut sfnt_table_data,
                FlushDecompress::None
            ).unwrap();

            println!("total_bytes_in: {}, total_bytes_out: {}",
                     decompressor.total_in(), decompressor.total_out());
        } else {
            sfnt_table_data.extend_from_slice(source_slice);

            println!("total_bytes_in: {}, total_bytes_out: {}",
                     source_slice.len(), sfnt_table_data.len());
        }

        let snft_table_record = SfntTableRecord {
            table_tag: table_dir_entry.tag,
            checksum: table_dir_entry.orig_checksum,
            offset: sfnt_table_offset as u32,
            length: table_dir_entry.orig_length
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

    assemble_sfnt_binary(sfnt_offset_table, sfnt_table_records_vec, sfnt_table_data_vec, decoded_data)
}