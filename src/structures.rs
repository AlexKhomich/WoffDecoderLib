use crate::utils::{u32_to_u8_array, u16_to_u8_array};

/// WOFF header (44 bytes length)
#[allow(dead_code)]
pub(crate) struct WoffHeader {
    // "magic number" - 0x774F4646 'wOFF'
    pub(crate) signature: u32,
    // The "sfnt version" of the input font
    pub(crate) flavor: u32,
    // Total size of the WOFF file
    pub(crate) length: u32,
    // Number of entries in directory of font tables
    pub(crate) num_tables: u16,
    // Reserved; set to zero
    pub(crate) reserved: u16,
    // Total size needed for the uncompressed font data, including the sfnt header, directory,
    // and font tables (including padding)
    pub(crate) total_sfnt_size: u32,
    pub(crate) major_version: u16,
    pub(crate) minor_version: u16,
    // Offset to metadata block, from beginning of WOFF file
    pub(crate) meta_offset: u32,
    // Length of compressed metadata block
    pub(crate) meta_length: u32,
    // Uncompressed size of metadata block
    pub(crate) meta_orig_length: u32,
    // Offset to private data block, from beginning of WOFF file
    pub(crate) priv_offset: u32,
    // Length of private data block
    pub(crate) priv_length: u32,
}

/// WOFF table directory
pub(crate) struct WoffTableDirectoryEntry {
    // 4-byte sfnt table identifier
    pub(crate) tag: u32,
    // Offset to the data, from beginning of WOFF file
    pub(crate) offset: u32,
    // Length of the compressed data, excluding padding
    pub(crate) comp_length: u32,
    // Length of the uncompressed table, excluding padding
    pub(crate) orig_length: u32,
    // Checksum of the uncompressed table
    pub(crate) orig_checksum: u32,
}

/// WOFF offset table
pub(crate) struct SfntOffsetTable {
    // 0x00010000 or 0x4F54544F ('OTTO')
    pub(crate) version: u32,
    pub(crate) num_tables: u16,
    // (Maximum power of 2 <= numTables) x 16.
    pub(crate) search_range: u16,
    // Log2(maximum power of 2 <= numTables).
    pub(crate) entry_selector: u16,
    // NumTables x 16-searchRange.
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
    pub(crate) table_tag: u32,
    pub(crate) checksum: u32,
    // Offset from beginning of TrueType font file.
    pub(crate) offset: u32,
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
    // Minor version number of the font header table — set to 0
    pub(crate) minor_version: u16,
    // Set by font manufacturer
    pub(crate) font_revision: u32,
    pub(crate) check_sum_adjustment: u32,
    // Set to 0x5F0F3CF5 "OTTO"
    pub(crate) magic_number: u32,
    pub(crate) flags: u16,
    // Set to a value from 16 to 16384. Any value in this range is valid
    pub(crate) units_per_em: u16,
    // Number of seconds since 12:00 midnight that started January 1st 1904 in GMT/UTC time zone.
    // 64-bit integer
    pub(crate) created: [u32; 2],
    // Number of seconds since 12:00 midnight that started January 1st 1904 in GMT/UTC time zone.
    // 64-bit integer
    pub(crate) modified: [u32; 2],
    // For all glyph bounding boxes
    pub(crate) x_min: i16,
    pub(crate) y_min: i16,
    pub(crate) x_max: i16,
    pub(crate) y_max: i16,
    // Bit 0: Bold (if set to 1); Bit 1: Italic (if set to 1) Bit 2: Underline (if set to 1)
    // Bit 3: Outline (if set to 1) Bit 4: Shadow (if set to 1) Bit 5: Condensed (if set to 1)
    // Bit 6: Extended (if set to 1) Bits 7–15: Reserved (set to 0).
    pub(crate) mac_style: u16,
    // Smallest readable size in pixels.
    pub(crate) lowest_rec_ppem: u16,
    // Deprecated. Only strongly left to right but also contains neutrals
    pub(crate) font_direction_hint: i16,
    // 0 for short offsets (Offset16), 1 for long (Offset32).
    pub(crate) index_to_loc_format: i16,
    // 0 for current format.
    pub(crate) glyph_data_format: i16,
}