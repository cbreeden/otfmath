use std::io::Read;
use std::io::SeekFrom;
use std::io::Seek;

use byteorder::{BigEndian, ReadBytesExt};
use ::constants::MathValueRecord;

#[derive(Debug, Clone, Copy)]
pub struct MathGlyphInfoTable {
    pub math_italics_correction: u64,
    pub math_top_accent: u64,
    pub extended_shape_coverage: u64,
    pub math_kern_info: u64,
}

impl MathGlyphInfoTable {
    pub fn read<T: Read>(rdr: &mut T) -> Result<MathGlyphInfoTable, String> {
        Ok(MathGlyphInfoTable {
            math_italics_correction: rdr.read_u16::<BigEndian>().unwrap() as u64,
            math_top_accent: rdr.read_u16::<BigEndian>().unwrap() as u64,
            extended_shape_coverage: rdr.read_u16::<BigEndian>().unwrap() as u64,
            math_kern_info: rdr.read_u16::<BigEndian>().unwrap() as u64,
        })
    } 
}

#[derive(Debug)]
pub struct MathItalicsCorrectionTable {
    pub offset: u64,
    pub count: u16,
    pub corrections: Vec<MathValueRecord>,
}

impl MathItalicsCorrectionTable {
    pub fn read<T: Read + Seek>(rdr: &mut T) -> Result<MathItalicsCorrectionTable, String> {
        let offset = rdr.read_u16::<BigEndian>().unwrap() as u64;
        let count  = rdr.read_u16::<BigEndian>().unwrap();

        let _ = rdr.seek(SeekFrom::Current(-4));
        let mut corrections: Vec<MathValueRecord> = Vec::with_capacity(count as usize);
        for _ in 0..count + 1 {
            corrections.push(MathValueRecord{
                value: rdr.read_i16::<BigEndian>().unwrap(),
                device_table: rdr.read_u16::<BigEndian>().unwrap(),
            });
        }

        Ok(MathItalicsCorrectionTable {
            offset: offset,
            count: count,
            corrections: corrections,
        })
    }
}

#[derive(Debug)]
pub struct MathTopAccentTable {
    pub offset: u64,
    pub count: u16,
    pub accents: Vec<MathValueRecord>,
}

impl MathTopAccentTable {
    pub fn read<T: Read + Seek>(rdr: &mut T) -> Result<MathTopAccentTable, String> {
        let offset = rdr.read_u16::<BigEndian>().unwrap() as u64;
        let count  = rdr.read_u16::<BigEndian>().unwrap();
        
        let _ = rdr.seek(SeekFrom::Current(-4));
        let mut accents: Vec<MathValueRecord> = Vec::with_capacity(count as usize);
        for _ in 0..count + 1 {
            accents.push(MathValueRecord{
                value: rdr.read_i16::<BigEndian>().unwrap(),
                device_table: rdr.read_u16::<BigEndian>().unwrap(),
            });
        }

        Ok(MathTopAccentTable {
            offset: offset,
            count: count,
            accents: accents,
        })
    }
}