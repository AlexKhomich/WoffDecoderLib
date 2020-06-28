use crate::utils::{u32_to_u8_array, u16_to_u8_array};

/// WOFF header (44 bytes length)
#[allow(dead_code)]
pub(crate) struct WoffHeader {
    // "magic number" - 0x774F4646 'wOFF'
    pub(crate) signature: u32,
    //  (crate)The "sfnt version" of the input font
    pub(crate) flavor: u32,
    //  (crate)Total size of the WOFF file
    pub(crate) length: u32,
    //  (crate)Number of entries in directory of font tables
    pub(crate) num_tables: u16,
    //  (crate)Reserved; set to zero
    pub(crate) reserved: u16,
    //  (crate)Total size needed for the uncompressed font data, including the sfnt header, directory, and font tables (including padding)
    pub(crate) total_sfnt_size: u32,
    //  (crate)Major version of the WOFF file
    pub(crate) major_version: u16,
    //  (crate)Minor version of the WOFF file
    pub(crate) minor_version: u16,
    //  (crate)Offset to metadata block, from beginning of WOFF file
    pub(crate) meta_offset: u32,
    //  (crate)Length of compressed metadata block
    pub(crate) meta_length: u32,
    //  (crate)Uncompressed size of metadata block
    pub(crate) meta_orig_length: u32,
    //  (crate)Offset to private data block, from beginning of WOFF file
    pub(crate) priv_offset: u32,
    //  (crate)Length of private data block
    pub(crate) priv_length: u32,
}

/// WOFF table directory
pub(crate) struct WoffTableDirectoryEntry {
    // 4-byte sfnt table identifier
    pub(crate) tag: u32,
    // (crate)Offset to the data, from beginning of WOFF file
    pub(crate) offset: u32,
    // (crate)Length of the compressed data, excluding padding
    pub(crate) comp_length: u32,
    // (crate)Length of the uncompressed table, excluding padding
    pub(crate) orig_length: u32,
    // (crate)Checksum of the uncompressed table
    pub(crate) orig_checksum: u32,
}

/// WOFF offset table
pub(crate) struct SfntOffsetTable {
    // 0x00010000 or 0x4F54544F ('OTTO')
    pub(crate) version: u32,
    // (crate)Number of tables.
    pub(crate) num_tables: u16,
    // (crate)(Maximum power of 2 <= numTables) x 16.
    pub(crate) search_range: u16,
    // (crate)Log2(maximum power of 2 <= numTables).
    pub(crate) entry_selector: u16,
    // (crate)NumTables x 16-searchRange.
    pub(crate) range_shift: u16,
}

impl SfntOffsetTable {
    pub(crate) fn transform_to_u8_vec(&self) -> Vec<u8> {
        let mut result_vec: Vec<u8> = Vec::with_capacity(12);
        result_vec.append(&mut u32_to_u8_array(self.version).to_vec());
        result_vec.append(&mut u16_to_u8_array(self.num_tables).to_vec());
        result_vec.append(&mut u16_to_u8_array(self.search_range).to_vec());
        result_vec.append(&mut u16_to_u8_array(self.entry_selector).to_vec());
        result_vec.append(&mut u16_to_u8_array(self.range_shift).to_vec());
        result_vec
    }
}

/// SFNT table record
pub(crate) struct SfntTableRecord {
    // Table identifier.
    pub(crate) table_tag: u32,
    // (crate)CheckSum for this table.
    pub(crate) checksum: u32,
    // (crate)Offset from beginning of TrueType font file.
    pub(crate) offset: u32,
    // (crate)Length of this table.
    pub(crate) length: u32,
}

impl SfntTableRecord {
    pub(crate) fn transform_to_u8_vec(&self) -> Vec<u8> {
    let mut result_vec: Vec<u8> = Vec::with_capacity(16);
    result_vec.append(&mut u32_to_u8_array(self.table_tag).to_vec());
    result_vec.append(&mut u32_to_u8_array(self.checksum).to_vec());
    result_vec.append(&mut u32_to_u8_array(self.offset).to_vec());
    result_vec.append(&mut u32_to_u8_array(self.length).to_vec());
    result_vec
    }
}

/// SFNT header table
#[allow(dead_code)]
pub(crate) struct SfntHeaderTable {
    // Major version number of the font header table — set to 1
    pub(crate) major_version: u16,
    // (crate)Minor version number of the font header table — set to 0
    pub(crate) minor_version: u16,
    // (crate)Set by font manufacturer
    pub(crate) font_revision: u32,
    // (crate)Check sum
    pub(crate) check_sum_adjustment: u32,
    // (crate)Set to 0x5F0F3CF5 "OTTO"
    pub(crate) magic_number: u32,
    // (crate)Flags
    pub(crate) flags: u16,
    // (crate)Set to a value from 16 to 16384. Any value in this range is valid
    pub(crate) units_per_em: u16,
    // (crate)Number of seconds since 12:00 midnight that started January 1st 1904 in GMT/UTC time zone. 64-bit integer
    pub(crate) created: [u32; 2],
    // (crate)Number of seconds since 12:00 midnight that started January 1st 1904 in GMT/UTC time zone. 64-bit integer
    pub(crate) modified: [u32; 2],
    // (crate)For all glyph bounding boxes
    pub(crate) x_min: i16,
    // (crate)For all glyph bounding boxes
    pub(crate) y_min: i16,
    // (crate)For all glyph bounding boxes
    pub(crate) x_max: i16,
    // (crate)For all glyph bounding boxes
    pub(crate) y_max: i16,
    // (crate)Bit 0: Bold (if set to 1); Bit 1: Italic (if set to 1) Bit 2: Underline (if set to 1) Bit 3: Outline (if set to 1) Bit 4: Shadow (if set to 1) Bit 5: Condensed (if set to 1) Bit 6: Extended (if set to 1) Bits 7–15: Reserved (set to 0).
    pub(crate) mac_style: u16,
    // (crate)Smallest readable size in pixels.
    pub(crate) lowest_rec_ppem: u16,
    // (crate)Deprecated. Only strongly left to right but also contains neutrals
    pub(crate) font_direction_hint: i16,
    // (crate)0 for short offsets (Offset16), 1 for long (Offset32).
    pub(crate) index_to_loc_format: i16,
    // (crate)0 for current format.
    pub(crate) glyph_data_format: i16,
}