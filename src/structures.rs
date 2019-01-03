/// Base range structure
pub trait Range {
    fn get_start_offset(&self) -> usize;
    fn get_end_offset(&self) -> usize;
    fn get_length(&self) -> usize;
}

struct BaseRange {
    start_byte: usize,
    end_byte: usize,
    size: usize,
}

pub struct WoffHeaderRange {
    range: Box<BaseRange>,
}

impl Range for WoffHeaderRange {
    fn get_start_offset(&self) -> usize {
        self.range.start_byte
    }

    fn get_end_offset(&self) -> usize {
        self.range.end_byte
    }

    fn get_length(&self) -> usize {
        self.range.size
    }
}

impl WoffHeaderRange {
    pub fn get_signature_range() -> Self {
        WoffHeaderRange { range: Box::new(BaseRange { start_byte: 0, end_byte: 4, size: 4 }) }
    }

    pub fn get_flavor_range() -> Self {
        WoffHeaderRange { range: Box::new(BaseRange { start_byte: 4, end_byte: 8, size: 4 }) }
    }

    pub fn get_length_range() -> Self {
        WoffHeaderRange { range: Box::new(BaseRange { start_byte: 8, end_byte: 12, size: 4 }) }
    }

    pub fn get_num_tables_range() -> Self {
        WoffHeaderRange { range: Box::new(BaseRange { start_byte: 12, end_byte: 14, size: 2 }) }
    }

    pub fn get_total_sfnt_size_range() -> Self {
        WoffHeaderRange { range: Box::new(BaseRange { start_byte: 16, end_byte: 20, size: 4 }) }
    }

    pub fn get_major_version_range() -> Self {
        WoffHeaderRange { range: Box::new(BaseRange { start_byte: 20, end_byte: 22, size: 2 }) }
    }

    pub fn get_minor_version_range() -> Self {
        WoffHeaderRange { range: Box::new(BaseRange { start_byte: 22, end_byte: 24, size: 2 }) }
    }

    pub fn get_meta_offset_range() -> Self {
        WoffHeaderRange { range: Box::new(BaseRange { start_byte: 24, end_byte: 28, size: 4 }) }
    }

    pub fn get_meta_length_range() -> Self {
        WoffHeaderRange { range: Box::new(BaseRange { start_byte: 28, end_byte: 32, size: 4 }) }
    }

    pub fn get_meta_orgig_length_range() -> Self {
        WoffHeaderRange { range: Box::new(BaseRange { start_byte: 32, end_byte: 36, size: 4 }) }
    }

    pub fn get_priv_offset_range() -> Self {
        WoffHeaderRange { range: Box::new(BaseRange { start_byte: 36, end_byte: 40, size: 4 }) }
    }

    pub fn get_priv_length_range() -> Self {
        WoffHeaderRange { range: Box::new(BaseRange { start_byte: 40, end_byte: 44, size: 4 }) }
    }
}

pub struct WoffTableDirectoryEntryRange {
    range: Box<BaseRange>
}

impl Range for WoffTableDirectoryEntryRange {
    fn get_start_offset(&self) -> usize {
        self.range.start_byte
    }

    fn get_end_offset(&self) -> usize {
        self.range.end_byte
    }

    fn get_length(&self) -> usize {
        self.range.size
    }
}

impl WoffTableDirectoryEntryRange {
    pub fn construct_tag_range(start_byte: usize, len: usize) -> Self {
        WoffTableDirectoryEntryRange { range: Box::new(
            BaseRange { start_byte, end_byte: start_byte + len , size: len }
        ) }
    }

    pub fn construct_offset_range(mut start_byte: usize, len: usize) -> Self {
        start_byte += 4;
        WoffTableDirectoryEntryRange { range: Box::new(
            BaseRange { start_byte, end_byte: start_byte + len, size: len }
        ) }
    }

    pub fn construct_comp_length(mut start_byte: usize, len: usize) -> Self {
        start_byte += 8;
        WoffTableDirectoryEntryRange { range: Box::new(
            BaseRange { start_byte, end_byte: start_byte + len, size: len }
        ) }
    }

    pub fn construct_orig_length(mut start_byte: usize, len: usize) -> Self {
        start_byte += 12;
        WoffTableDirectoryEntryRange { range: Box::new(
            BaseRange { start_byte, end_byte: start_byte + len, size: len }
        ) }
    }

    pub fn construct_orig_checksum(mut start_byte: usize, len: usize) -> Self {
        start_byte += 16;
        WoffTableDirectoryEntryRange { range: Box::new(
            BaseRange { start_byte, end_byte: start_byte + len, size: len }
        ) }
    }
}


/// WOFF header (44 bytes length)
pub struct WoffHeader {
    signature: u32,
    // "magic number" - 0x774F4646 'wOFF'
    pub flavor: u32,
    // The "sfnt version" of the input font
    length: u32,
    // Total size of the WOFF file
    pub num_tables: u16,
    // Number of entries in directory of font tables
    reserved: u16,
    // Reserved; set to zero
    total_sfnt_size: u32,
    // Total size needed for the uncompressed font data, including the sfnt header, directory, and font tables (including padding)
    major_version: u16,
    // Major version of the WOFF file
    minor_version: u16,
    // Minor version of the WOFF file
    meta_offset: u32,
    // Offset to metadata block, from beginning of WOFF file
    meta_length: u32,
    // Length of compressed metadata block
    meta_orig_length: u32,
    // Uncompressed size of metadata block
    priv_offset: u32,
    // Offset to private data block, from beginning of WOFF file
    priv_length: u32, // Length of private data block
}


/// Builder for WOFF header
pub struct WoffHeaderBuilder {
    signature: u32,
    flavor: u32,
    length: u32,
    num_tables: u16,
    reserved: u16,
    total_sfnt_size: u32,
    major_version: u16,
    minor_version: u16,
    meta_offset: u32,
    meta_length: u32,
    meta_orig_length: u32,
    priv_offset: u32,
    priv_length: u32,
}

/// WOFF table directory
pub struct WoffTableDirectoryEntry {
    pub tag: u32,
    // 4-byte sfnt table identifier
    pub offset: u32,
    // Offset to the data, from beginning of WOFF file
    pub comp_length: u32,
    // Length of the compressed data, excluding padding
    pub orig_length: u32,
    // Length of the uncompressed table, excluding padding
    pub orig_checksum: u32, // Checksum of the uncompressed table
}


/// WOFF table directory builder
pub struct WoffTableDirectoryEntryBuilder {
    tag: u32,
    offset: u32,
    comp_length: u32,
    orig_length: u32,
    orig_checksum: u32,
}

/// Woff header builder implementation
impl WoffHeaderBuilder {
    pub fn new() -> WoffHeaderBuilder {
        WoffHeaderBuilder {
            signature: 0x774F4646,
            flavor: 0,
            length: 0,
            num_tables: 0,
            reserved: 0,
            total_sfnt_size: 0,
            major_version: 0,
            minor_version: 0,
            meta_offset: 0,
            meta_length: 0,
            meta_orig_length: 0,
            priv_offset: 0,
            priv_length: 0,
        }
    }

    pub fn set_signature(&mut self, signature: u32) -> &mut WoffHeaderBuilder {
        if self.signature != signature {
            panic!("WOFF file signature has incorrect value: {}", signature)
        } else {
            self.signature = signature;
            self
        }
    }

    pub fn set_flavor(&mut self, flavor: u32) -> &mut WoffHeaderBuilder {
        self.flavor = flavor;
        self
    }

    pub fn set_length(&mut self, length: u32) -> &mut WoffHeaderBuilder {
        self.length = length;
        self
    }

    pub fn set_num_tables(&mut self, num_tables: u16) -> &mut WoffHeaderBuilder {
        self.num_tables = num_tables;
        self
    }

    pub fn set_total_sfnt_size(&mut self, total_sfnt_size: u32) -> &mut WoffHeaderBuilder {
        self.total_sfnt_size = total_sfnt_size;
        self
    }

    pub fn set_major_version(&mut self, major_version: u16) -> &mut WoffHeaderBuilder {
        self.major_version = major_version;
        self
    }

    pub fn set_minor_version(&mut self, minor_version: u16) -> &mut WoffHeaderBuilder {
        self.minor_version = minor_version;
        self
    }

    pub fn set_meta_offset(&mut self, meta_offset: u32) -> &mut WoffHeaderBuilder {
        self.meta_offset = meta_offset;
        self
    }

    pub fn set_meta_length(&mut self, meta_length: u32) -> &mut WoffHeaderBuilder {
        self.meta_length = meta_length;
        self
    }

    pub fn set_meta_orig_length(&mut self, meta_orig_length: u32) -> &mut WoffHeaderBuilder {
        self.meta_orig_length = meta_orig_length;
        self
    }

    pub fn set_priv_offset(&mut self, priv_offset: u32) -> &mut WoffHeaderBuilder {
        self.priv_offset = priv_offset;
        self
    }

    pub fn set_priv_length(&mut self, priv_length: u32) -> &mut WoffHeaderBuilder {
        self.priv_length = priv_length;
        self
    }

    pub fn build(&self) -> WoffHeader {
        WoffHeader {
            signature: self.signature,
            flavor: self.flavor,
            length: self.length,
            num_tables: self.num_tables,
            reserved: self.reserved,
            total_sfnt_size: self.total_sfnt_size,
            major_version: self.major_version,
            minor_version: self.minor_version,
            meta_offset: self.meta_offset,
            meta_length: self.meta_length,
            meta_orig_length: self.meta_orig_length,
            priv_offset: self.priv_offset,
            priv_length: self.priv_length,
        }
    }
}

/// Woff header tablee    builder implementation
impl WoffTableDirectoryEntryBuilder {
    pub fn new() -> WoffTableDirectoryEntryBuilder {
        WoffTableDirectoryEntryBuilder {
            tag: 0,
            offset: 0,
            comp_length: 0,
            orig_length: 0,
            orig_checksum: 0,
        }
    }
    pub fn set_tag(&mut self, tag: u32) -> &WoffTableDirectoryEntryBuilder {
        self.tag = tag;
        self
    }

    pub fn set_offset(&mut self, offset: u32) -> &WoffTableDirectoryEntryBuilder {
        self.offset = offset;
        self
    }

    pub fn set_comp_length(&mut self, comp_length: u32) -> &WoffTableDirectoryEntryBuilder {
        self.comp_length = comp_length;
        self
    }

    pub fn set_orig_length(&mut self, orig_length: u32) -> &WoffTableDirectoryEntryBuilder {
        self.orig_length = orig_length;
        self
    }

    pub fn set_orig_checksum(&mut self, orig_checksum: u32) -> &WoffTableDirectoryEntryBuilder {
        self.orig_checksum = orig_checksum;
        self
    }

    pub fn build(&self) -> WoffTableDirectoryEntry {
        WoffTableDirectoryEntry {
            tag: self.tag,
            offset: self.offset,
            comp_length: self.comp_length,
            orig_length: self.orig_length,
            orig_checksum: self.orig_checksum,
        }
    }
}


/// WOFF offset table
pub struct SfntOffsetTable {
    version: u32,
    // 0x00010000 or 0x4F54544F ('OTTO')
    pub num_tables: u16,
    // Number of tables.
    search_range: u16,
    // (Maximum power of 2 <= numTables) x 16.
    entry_selector: u16,
    // Log2(maximum power of 2 <= numTables).
    range_shift: u16, // NumTables x 16-searchRange.
}

/// WOFF offset table builder
pub struct SfntOffsetTableBuilder {
    version: u32,
    num_tables: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
}

impl SfntOffsetTableBuilder {
    pub fn new() -> Self {
        SfntOffsetTableBuilder {
            version: 0,
            num_tables: 0,
            search_range: 0,
            entry_selector: 0,
            range_shift: 0,
        }
    }

    pub fn set_version(&mut self, version: u32) -> &Self {
        self.version = version;
        self
    }

    pub fn set_num_tables(&mut self, num_tables: u16) -> &Self {
        self.num_tables = num_tables;
        self
    }

    pub fn set_search_range(&mut self, search_range: u16) -> &Self {
        self.search_range = search_range;
        self
    }

    pub fn set_entry_selector(&mut self, entry_selector: u16) -> &Self {
        self.entry_selector = entry_selector;
        self
    }

    pub fn set_range_shift(&mut self, range_shift: u16) -> &Self {
        self.range_shift = range_shift;
        self
    }

    pub fn build(&self) -> SfntOffsetTable {
        SfntOffsetTable {
            version: self.version,
            num_tables: self.num_tables,
            search_range: self.search_range,
            entry_selector: self.entry_selector,
            range_shift: self.range_shift,
        }
    }
}

/// SFNT table record
pub struct SfntTableRecord {
    table_tag: u32,
    // Table identifier.
    checksum: u32,
    // CheckSum for this table.
    offset: u32,
    // Offset from beginning of TrueType font file.
    length: u32, // Length of this table.
}

/// SFNT table record builder
pub struct SfntTableRecordBuilder {
    table_tag: u32,
    checksum: u32,
    offset: u32,
    length: u32,
}

impl SfntTableRecordBuilder {
    pub fn new() -> Self {
        SfntTableRecordBuilder {
            table_tag: 0,
            checksum: 0,
            offset: 0,
            length: 0
        }
    }

    pub fn set_table_tag(&mut self, table_tag: u32) -> &Self {
        self.table_tag = table_tag;
        self
    }

    pub fn set_checksum(&mut self, checksum: u32) -> &Self {
        self.checksum = checksum;
        self
    }

    pub fn set_offset(&mut self, offset: u32) -> &Self {
        self.offset = offset;
        self
    }

    pub fn set_length(&mut self, length: u32) -> &Self {
        self.length = length;
        self
    }

    pub fn build(&self) -> SfntTableRecord {
        SfntTableRecord {
            table_tag: self.table_tag,
            checksum: self.checksum,
            offset: self.offset,
            length: self.length,
        }
    }
}

/// SFNT header table
pub struct SfntHeaderTable {
    major_version: u16,
    // Major version number of the font header table — set to 1
    minor_version: u16,
    // Minor version number of the font header table — set to 0
    font_revision: u32,
    // Set by font manufacturer
    check_sum_adjustment: u32,
    // Check sum
    magic_number: u32,
    // Set to 0x5F0F3CF5 "OTTO"
    flags: u16,
    // Flags
    units_per_em: u16,
    // Set to a value from 16 to 16384. Any value in this range is valid
    created: [u32; 2],
    // Number of seconds since 12:00 midnight that started January 1st 1904 in GMT/UTC time zone. 64-bit integer
    modified: [u32; 2],
    // Number of seconds since 12:00 midnight that started January 1st 1904 in GMT/UTC time zone. 64-bit integer
    x_min: i16,
    // For all glyph bounding boxes
    y_min: i16,
    // For all glyph bounding boxes
    x_max: i16,
    // For all glyph bounding boxes
    y_max: i16,
    // For all glyph bounding boxes
    mac_style: u16,
    // Bit 0: Bold (if set to 1); Bit 1: Italic (if set to 1) Bit 2: Underline (if set to 1) Bit 3: Outline (if set to 1) Bit 4: Shadow (if set to 1) Bit 5: Condensed (if set to 1) Bit 6: Extended (if set to 1) Bits 7–15: Reserved (set to 0).
    lowest_rec_ppem: u16,
    // Smallest readable size in pixels.
    font_direction_hint: i16,
    // Deprecated. Only strongly left to right but also contains neutrals
    index_to_loc_format: i16,
    // 0 for short offsets (Offset16), 1 for long (Offset32).
    glyph_data_format: i16, // 0 for current format.
}

/// SFNT header table builder
pub struct SfntHeaderTableBuilder {
    major_version: u16,
    minor_version: u16,
    font_revision: u32,
    check_sum_adjustment: u32,
    magic_number: u32,
    flags: u16,
    units_per_em: u16,
    created: [u32; 2],
    modified: [u32; 2],
    x_min: i16,
    y_min: i16,
    x_max: i16,
    y_max: i16,
    mac_style: u16,
    lowest_rec_ppem: u16,
    font_direction_hint: i16,
    index_to_loc_format: i16,
    glyph_data_format: i16,
}




