use byteorder::{BigEndian, ReadBytesExt};
//use std::io::Result;
use std::fmt;

/// SfntVersion - Spline Font Version
/// 
/// For an OpenType font this should be either 1 or OTTO.
/// TrueType Fonts should use Version1 and CFF fonts should use OTTO.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SfntVersion {
    Version1,
    OTTO,
}

///
/// `offset_tables`: Must be sorted in ascending order by `tag`
/// const OFFSET_TABLE_SIZE: usize = 96;
/// const RECORD_TABLE_SIZE: usize = 128;

#[derive(Debug)]
pub struct OffsetTable {
    pub sfnt_version: SfntVersion,
    pub num_tables: u16,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
    pub offset_tables: Vec<TableRecord>,
}

impl OffsetTable {
    pub fn read<T: ::std::io::Read>(rdr: &mut T) -> Result<OffsetTable, String> {
        let sfnt_version = match rdr.read_u32::<BigEndian>().unwrap() {
            0x00010000 => SfntVersion::Version1,

            // b'OTTO' as u32
            0x4f54544f => SfntVersion::OTTO,
            v @ _ => return Err(format!("Unsupported Splint Font Version {}", v)),
        };

        let num_tables = rdr.read_u16::<BigEndian>().unwrap();
        let search_range = rdr.read_u16::<BigEndian>().unwrap();
        let entry_selector = rdr.read_u16::<BigEndian>().unwrap();
        let range_shift = rdr.read_u16::<BigEndian>().unwrap();

        let mut offset_tables = Vec::<TableRecord>::with_capacity(num_tables as usize);
        for _ in 0..num_tables {
            offset_tables.push(TableRecord::read(&mut* rdr).unwrap());
        } 

        Ok(OffsetTable {
            sfnt_version: sfnt_version,
            num_tables: num_tables,
            search_range: search_range,
            entry_selector: entry_selector,
            range_shift: range_shift,
            offset_tables: offset_tables,
        })        
    }    
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Tag(pub [u8; 4]);

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &Tag(inner) = self;
        write!(f, "{}", ::std::str::from_utf8(&inner).unwrap())
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use the same as Display
        <Self as fmt::Display>::fmt(self, f)
    }
}

/// `Tag` a 4-byte identifier for the table.  Ie: "MATH", "GPOS", "GDEF", etc..
///       values must be between 32-126, and therefore valid utf8.
/// `check_sum` for particular record
/// `offset` from beginning of TT font file.
/// `length` length of table.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TableRecord {
    pub tag: Tag,
    check_sum: u32, 
    pub offset: u32,
    length: u32,
}

impl TableRecord {
    fn read<T: ::std::io::Read>(rdr: &mut T) -> Result<TableRecord, String> {
        let mut tag = [0u8; 4];
        rdr.read_exact(&mut tag).unwrap();

        Ok(TableRecord{
            tag: Tag(tag),
            check_sum: rdr.read_u32::<BigEndian>().unwrap(),
            offset: rdr.read_u32::<BigEndian>().unwrap(),
            length: rdr.read_u32::<BigEndian>().unwrap(),
        })
    }
}