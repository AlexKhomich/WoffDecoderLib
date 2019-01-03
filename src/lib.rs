extern crate byteorder;
extern crate flate2;

mod utils;
mod structures;

use crate::structures::*;
use crate::utils::*;

use std::mem::size_of;
use flate2::{Decompress, FlushDecompress};

/// Decode .woff file data to SFNT bytes
pub fn decode_from_file(path: &str, mut dest_buf: *mut u8) -> usize {
    let mut buf: Vec<u8> = vec![];
    read_file(path, &mut buf);
    dest_buf = decode_internal(&mut buf);

    // test data
    let mut result: Vec<u8> = vec![2, 5, 10, 20];
    dest_buf = result.as_mut_ptr();
    result.len()
    //end test data
}


/// Decode WOFF data to SFNT data
pub fn decode_from_data(source_buf: *const u8, woff_data_size: usize, mut dest_buf: *mut u8) -> usize {
    unimplemented!()
}

/// Function for creating woff header form raw data
fn create_woff_header(buf: &mut Vec<u8>) -> WoffHeader {

    let mut woff_header_builder = WoffHeaderBuilder::new();
    woff_header_builder.set_signature(read_u32_be(buf, Box::new(WoffHeaderRange::get_signature_range())));
    woff_header_builder.set_flavor(read_u32_be(buf, Box::new(WoffHeaderRange::get_flavor_range())));
    woff_header_builder.set_length(read_u32_be(buf, Box::new(WoffHeaderRange::get_length_range())));
    woff_header_builder.set_num_tables(read_u16_be(buf, Box::new(WoffHeaderRange::get_num_tables_range())));
    woff_header_builder.set_total_sfnt_size(read_u32_be(buf, Box::new(WoffHeaderRange::get_total_sfnt_size_range())));
    woff_header_builder.set_major_version(read_u16_be(buf, Box::new(WoffHeaderRange::get_major_version_range())));
    woff_header_builder.set_minor_version(read_u16_be(buf, Box::new(WoffHeaderRange::get_minor_version_range())));
    woff_header_builder.set_meta_offset(read_u32_be(buf, Box::new(WoffHeaderRange::get_meta_offset_range())));
    woff_header_builder.set_meta_length(read_u32_be(buf, Box::new(WoffHeaderRange::get_meta_length_range())));
    woff_header_builder.set_meta_orig_length(read_u32_be(buf, Box::new(WoffHeaderRange::get_meta_orgig_length_range())));
    woff_header_builder.set_priv_offset(read_u32_be(buf, Box::new(WoffHeaderRange::get_priv_offset_range())));
    woff_header_builder.set_priv_length(read_u32_be(buf, Box::new(WoffHeaderRange::get_priv_length_range())));
    woff_header_builder.build()
}

/// function for creating WOFF table directory entry structure
fn create_woff_table_dir_entry(buf: &mut Vec<u8>, next_table_offset: usize) -> WoffTableDirectoryEntry {
    let mut builder = WoffTableDirectoryEntryBuilder::new();
    builder.set_tag(
        read_u32_be(buf, Box::new(
            WoffTableDirectoryEntryRange::construct_tag_range(next_table_offset, 4)
        ))
    );
    builder.set_offset(
        read_u32_be(buf, Box::new(
            WoffTableDirectoryEntryRange::construct_offset_range(next_table_offset, 4)
        ))
    );
    builder.set_comp_length(
        read_u32_be(buf, Box::new(
            WoffTableDirectoryEntryRange::construct_comp_length(next_table_offset, 4)
        ))
    );
    builder.set_orig_length(
        read_u32_be(buf, Box::new(
            WoffTableDirectoryEntryRange::construct_orig_length(next_table_offset, 4)
        ))
    );
    builder.set_orig_checksum(
        read_u32_be(buf, Box::new(
            WoffTableDirectoryEntryRange::construct_orig_checksum(next_table_offset, 4)
        ))
    );
    builder.build()
}

fn decode_internal(mut buf: &mut Vec<u8>) -> *mut u8 {

    let sfnt_offset_table_size = size_of::<SfntOffsetTable>();
    let sfnt_table_record_size = size_of::<SfntTableRecord>();
    let woff_table_directory_size = size_of::<WoffTableDirectoryEntry>();
    let woff_header_size = size_of::<WoffHeader>();

    let woff_header = create_woff_header(buf);

    let search_range= calculate_search_range(woff_header.num_tables);
    let entry_selector = calculate_entry_selector(search_range);
    let range_shift = calculate_range_shift(woff_header.num_tables, search_range);

    let mut builder = SfntOffsetTableBuilder::new();
    builder.set_version(woff_header.flavor);
    builder.set_num_tables(woff_header.num_tables);
    builder.set_entry_selector(entry_selector);
    builder.set_search_range(search_range);
    builder.set_range_shift(range_shift);
    let sfnt_offset_table = builder.build();

    let sfnt_table_size = sfnt_offset_table.num_tables;
    let mut sfnt_table_offset = sfnt_offset_table_size;

    let mut woff_table_dir_entry_container: Vec<WoffTableDirectoryEntry> = vec![];

    // construct each SFNT table record
    for i in 0..sfnt_table_size as usize {
        let mut next_table_offset = woff_header_size + (i * woff_table_directory_size);
        let woff_table_dir_entry = create_woff_table_dir_entry(&mut buf, next_table_offset);
        woff_table_dir_entry_container.push(woff_table_dir_entry);
        sfnt_table_offset += sfnt_table_record_size
    }

    // sort all entries by tag
    woff_table_dir_entry_container.sort_by(
        |a, b| a.tag.cmp(&b.tag)
    );

    for i in 0..sfnt_table_size as usize {
        let table_dir_entry = &woff_table_dir_entry_container[i];
        let start_offset = table_dir_entry.offset as usize;
        let end_offset = (table_dir_entry.offset + table_dir_entry.comp_length) as usize;

        if table_dir_entry.orig_length != table_dir_entry.comp_length {

            let source = &buf[start_offset..end_offset];
            let mut dest_vec: Vec<u8> = Vec::with_capacity(table_dir_entry.orig_length as usize);

            let mut decompressor = Decompress::new(true);
            let status = decompressor.decompress_vec(source, &mut dest_vec, FlushDecompress::None).unwrap();

            let total_bytes_in = decompressor.total_in();
            let total_bytes_out = decompressor.total_out();
        } else {
            let source = &buf[start_offset..end_offset];
            let dest_vec: Vec<u8> = Vec::from(source);
            let len = dest_vec.len();
        }

        let mut builder = SfntTableRecordBuilder::new();
        builder.set_table_tag(table_dir_entry.tag);
        builder.set_checksum(table_dir_entry.orig_checksum);
        builder.set_offset(sfnt_table_offset as u32);
        builder.set_length(table_dir_entry.orig_length);
        let snft_table_record = builder.build();
    }

    unimplemented!()
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        std::mem::size_of::<T>(),
    )
}


