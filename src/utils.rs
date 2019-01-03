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
use std::fs::{File};
use std::io::{BufReader, Read};
use crate::structures::Range;

/// Reads data from file to buffer
pub fn read_file(path: &str, buf: &mut Vec<u8>) -> usize {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    reader.read_to_end(buf).unwrap()
}

/// Works functions for reading data
pub fn read_u32_be(val: &mut Vec<u8>, range: Box<Range>) -> u32 {
    let mut part_vec: Vec<u8> = vec![];
    part_vec.extend(val.get(range.get_start_offset()..range.get_end_offset()).unwrap());
    let mut rdr = Cursor::new(part_vec);
    let rez = rdr.read_u32::<BigEndian>().unwrap();
    rez
}

pub fn read_u16_be(val: &mut Vec<u8>, range: Box<Range>) -> u16 {
    let mut part_vec: Vec<u8> = vec![];
    part_vec.extend(val.get(range.get_start_offset()..range.get_end_offset()).unwrap());
    let mut rdr = Cursor::new(part_vec);
    let rez = rdr.read_u16::<BigEndian>().unwrap();
    rez
}

pub fn calculate_entry_selector(mut number: u16) -> u16 {
    let mut res: u16 = 0;
    while number > 16 {
        number >>= 1;
        res = res + 1;
    }
    res
}

pub fn calculate_range_shift(num_tables: u16, serach_range: u16) -> u16 {
    num_tables * 16 - serach_range
}

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