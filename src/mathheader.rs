use byteorder::{BigEndian, ReadBytesExt};

/// Version: Always set to 1 for now
/// Constats: offset to math constants table
/// glyph_info: same
/// math_variants: same

#[derive(Debug, Clone, Copy)]
pub struct MathHeaderTable {
    pub version: u32,
    pub math_constants: u16,
    pub math_glyph_info: u16,
    pub math_variants: u16,
}

impl MathHeaderTable {
    pub fn read<T: ::std::io::Read>(rdr: &mut T) -> Result<MathHeaderTable, String> {
        Ok(MathHeaderTable {
            version: rdr.read_u32::<BigEndian>().unwrap(),
            math_constants: rdr.read_u16::<BigEndian>().unwrap(),
            math_glyph_info: rdr.read_u16::<BigEndian>().unwrap(),
            math_variants: rdr.read_u16::<BigEndian>().unwrap(),
        })
    } 
}