/* Tables tags list

/* -- Tags for required TrueType tables */
cmapTag = 0x636D6170; // 'cmap'
glyfTag = 0x676C7966; // 'glyf'
headTag = 0x68656164; // 'head'
hheaTag = 0x68686561; // 'hhea'
hmtxTag = 0x686D7478; // 'hmtx'
locaTag = 0x6C6F6361; // 'loca'
maxpTag = 0x6D617870; // 'maxp'
nameTag = 0x6E616D65; // 'name'
postTag = 0x706F7374; // 'post'
prepTag = 0x70726570; // 'prep'
os_2Tag = 0x4F532F32; // 'OS/2'

 /* -- Tags for opentype related tables */
GDEFTag = 0x47444546; // 'GDEF'
GPOSTag = 0x47504F53; // 'GPOS'
GSUBTag = 0x47535542; // 'GSUB'
mortTag = 0x6D6F7274; // 'mort'

 /* -- Tags for non-standard tables */
fdscTag = 0x66647363; // 'fdsc' - gxFont descriptor
fvarTag = 0x66766172; // 'fvar' - gxFont variations
featTag = 0x66656174; // 'feat' - layout features
EBLCTag = 0x45424C43; // 'EBLC' - embedded bitmaps
gaspTag = 0x67617370; // 'gasp' - hint/smooth sizes

    /* --  Other tags */
ttcfTag = 0x74746366; // 'ttcf' - TTC file
v1ttTag = 0x00010000; // 'v1tt' - Version 1 TT font
trueTag = 0x74727565; // 'true' - Version 2 TT font
ottoTag = 0x4f54544f; // 'otto' - OpenType font
*/

use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::{BufReader, Read};
use crate::structures::Range;

#[cfg(test)]
mod tests {}

/// Reads data from file to buffer
pub fn read_file(path: &str, buf: &mut Vec<u8>) -> usize {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    reader.read_to_end(buf).unwrap()
}

/// This one reads unsigned 32-bits value in big endian order
pub fn read_u32_be(val: &mut Vec<u8>, range: Box<Range>) -> u32 {
    let mut part_vec: Vec<u8> = vec![];
    part_vec.extend(val.get(range.get_start_offset()..range.get_end_offset()).unwrap());
    let mut rdr = Cursor::new(part_vec);
    let rez = rdr.read_u32::<BigEndian>().unwrap();
    rez
}

/// This one reads unsigned 16-bits value in big endian order
pub fn read_u16_be(val: &mut Vec<u8>, range: Box<Range>) -> u16 {
    let mut part_vec: Vec<u8> = vec![];
    part_vec.extend(val.get(range.get_start_offset()..range.get_end_offset()).unwrap());
    let mut rdr = Cursor::new(part_vec);
    let rez = rdr.read_u16::<BigEndian>().unwrap();
    rez
}

/// Calculates the entrySelector that is log2(maximum power of 2 <= numTables).
/// It tells how many iterations of the search loop are needed. (i.e. how many times to cut the range in half)
pub fn calculate_entry_selector(mut number: u16) -> u16 {
    let mut res: u16 = 0;
    while number > 16 {
        number >>= 1;
        res = res + 1;
    }
    res
}

/// Calculates rangeShift (numTables*16-searchRange)
pub fn calculate_range_shift(num_tables: u16, search_range: u16) -> u16 {
    num_tables * 16 - search_range
}

/// Calculates search range for every SFNT data table.
/// This one has to be (maximum power of 2 <= numTables)*16.
///  For example:
///  result = Math.pow(2, Math.floor(Math.log(num_ables) / Math.log(2)));
///  result * 16;
pub fn calculate_search_range(num_tables: u16) -> u16 {
    let mut sr = num_tables;
    sr = sr | (sr >> 1);
    sr = sr | (sr >> 2);
    sr = sr | (sr >> 4);
    sr = sr | (sr >> 8);
    sr &= !(sr >> 1);
    sr *= 16;
    sr
}

/// Calculates padded length for structure that has to be aligned by 4-bytes.
pub fn calculate_padded_len(orig_len: u32, sfnt_table_data_len: usize) -> u32 {
    let aligned_len = (orig_len + 3) & !3;
    aligned_len - sfnt_table_data_len as u32
}

/// Works only with the little endian order.
/// Result slice will be in the little endian order!
#[allow(dead_code)]
pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        std::mem::size_of::<T>(),
    )
}

/// Transforms unsigned 32-bits number to array of bytes.
/// Result vector contains values in big endian order!
pub fn u32_to_u8_array(x: u32) -> [u8; 4] {
    let mut result: [u8; 4] = [0; 4];
    result[0] = ((x >> 24) & 0xff) as u8;
    result[1] = ((x >> 16) & 0xff) as u8;
    result[2] = ((x >> 8) & 0xff) as u8;
    result[3] = (x & 0xff) as u8;
    result
}

/// Transforms unsigned 16-bits number to array of bytes.
/// Result vector contains values in big endian order!
pub fn u16_to_u8_array(x: u16) -> [u8; 2] {
    let mut result: [u8; 2] = [0; 2];
    result[0] = ((x >> 8) & 0xff) as u8;
    result[1] = (x & 0xff) as u8;
    result
}

/// Transforms unsigned 32-bits number to vector of bytes.
/// Result vector contains values in big endian order!
#[allow(dead_code)]
pub fn transform_u32_to_u8_vec(x: u32) -> Vec<u8> {
    let result: [u8; 4] = x.to_be_bytes();
    result.to_vec()
}

/// Transforms unsigned 16-bits number to vector of bytes.
/// Result vector contains values in big endian order!
#[allow(dead_code)]
pub fn transform_u16_to_u8_vec(x: u16) -> Vec<u8> {
    let mut result_vec: Vec<u8> = Vec::with_capacity(2);
    result_vec.push(((x >> 8) & 0xff) as u8);
    result_vec.push((x & 0xff) as u8);
    result_vec
}