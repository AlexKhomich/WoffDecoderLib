use crate::utils::{u32_to_u8_array, u16_to_u8_array};

/// WOFF header (44 bytes length)
pub struct WoffHeader {
    // "magic number" - 0x774F4646 'wOFF'
    pub signature: u32,
    // The "sfnt version" of the input font
    pub flavor: u32,
    // Total size of the WOFF file
    pub length: u32,
    // Number of entries in directory of font tables
    pub num_tables: u16,
    // Reserved; set to zero
    pub reserved: u16,
    // Total size needed for the uncompressed font data, including the sfnt header, directory, and font tables (including padding)
    pub total_sfnt_size: u32,
    // Major version of the WOFF file
    pub major_version: u16,
    // Minor version of the WOFF file
    pub minor_version: u16,
    // Offset to metadata block, from beginning of WOFF file
    pub meta_offset: u32,
    // Length of compressed metadata block
    pub meta_length: u32,
    // Uncompressed size of metadata block
    pub meta_orig_length: u32,
    // Offset to private data block, from beginning of WOFF file
    pub priv_offset: u32,
    // Length of private data block
    pub priv_length: u32,
}

/// WOFF table directory
pub struct WoffTableDirectoryEntry {
    // 4-byte sfnt table identifier
    pub tag: u32,
    // Offset to the data, from beginning of WOFF file
    pub offset: u32,
    // Length of the compressed data, excluding padding
    pub comp_length: u32,
    // Length of the uncompressed table, excluding padding
    pub orig_length: u32,
    // Checksum of the uncompressed table
    pub orig_checksum: u32,
}

/// WOFF offset table
pub struct SfntOffsetTable {
    // 0x00010000 or 0x4F54544F ('OTTO')
    pub version: u32,
    // Number of tables.
    pub num_tables: u16,
    // (Maximum power of 2 <= numTables) x 16.
    pub search_range: u16,
    // Log2(maximum power of 2 <= numTables).
    pub entry_selector: u16,
    // NumTables x 16-searchRange.
    pub range_shift: u16,
}

impl SfntOffsetTable {
    pub fn transform_to_u8_vec(&self) -> Vec<u8> {
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
pub struct SfntTableRecord {
    // Table identifier.
    pub table_tag: u32,
    // CheckSum for this table.
    pub checksum: u32,
    // Offset from beginning of TrueType font file.
    pub offset: u32,
    // Length of this table.
    pub length: u32,
}

impl SfntTableRecord {
    pub fn transform_to_u8_vec(&self) -> Vec<u8> {
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
pub struct SfntHeaderTable {
    // Major version number of the font header table — set to 1
    pub major_version: u16,
    // Minor version number of the font header table — set to 0
    pub minor_version: u16,
    // Set by font manufacturer
    pub font_revision: u32,
    // Check sum
    pub check_sum_adjustment: u32,
    // Set to 0x5F0F3CF5 "OTTO"
    pub magic_number: u32,
    // Flags
    pub flags: u16,
    // Set to a value from 16 to 16384. Any value in this range is valid
    pub units_per_em: u16,
    // Number of seconds since 12:00 midnight that started January 1st 1904 in GMT/UTC time zone. 64-bit integer
    pub created: [u32; 2],
    // Number of seconds since 12:00 midnight that started January 1st 1904 in GMT/UTC time zone. 64-bit integer
    pub modified: [u32; 2],
    // For all glyph bounding boxes
    pub x_min: i16,
    // For all glyph bounding boxes
    pub y_min: i16,
    // For all glyph bounding boxes
    pub x_max: i16,
    // For all glyph bounding boxes
    pub y_max: i16,
    // Bit 0: Bold (if set to 1); Bit 1: Italic (if set to 1) Bit 2: Underline (if set to 1) Bit 3: Outline (if set to 1) Bit 4: Shadow (if set to 1) Bit 5: Condensed (if set to 1) Bit 6: Extended (if set to 1) Bits 7–15: Reserved (set to 0).
    pub mac_style: u16,
    // Smallest readable size in pixels.
    pub lowest_rec_ppem: u16,
    // Deprecated. Only strongly left to right but also contains neutrals
    pub font_direction_hint: i16,
    // 0 for short offsets (Offset16), 1 for long (Offset32).
    pub index_to_loc_format: i16,
    // 0 for current format.
    pub glyph_data_format: i16,
}