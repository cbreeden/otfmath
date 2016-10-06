extern crate byteorder;
extern crate opentype;
extern crate truetype;

mod header;
mod mathheader;
mod constants;
mod glyphinfo;

use std::io::Cursor;
use std::io::Read;

use header::{ OffsetTable, Tag };
use mathheader::MathHeaderTable;
use constants::MathConstantTable;
use glyphinfo::{ MathGlyphInfoTable, MathItalicsCorrectionTable, MathTopAccentTable };



fn main() {
    let path = "latinmodern-math.otf";
    //let path = "xits-math.otf";
    let mut file = std::fs::File::open(path).unwrap();
    
    //for me
    let mut reader: Vec<u8> = Vec::new();
    file.read_to_end(&mut reader).unwrap();
    let mut cursor  = Cursor::new(reader);

    //ffor otf
    //let mut reader2 = std::fs::File::open(path).unwrap();
    //let mut font = Font::read(&mut reader2).unwrap();     
    //println!("{:#?}", font.offset_table);
    //let font_header: FontHeader = font.take(&mut reader2).unwrap().unwrap();    
    //println!("{:#?}", font_header);


    let header = OffsetTable::read(&mut cursor).unwrap();
    //println!("{:#?}", header);

    // Find math table
    let mut offset: Option<u64> = None;
    for table in header.offset_tables {
        let Tag(tag) = table.tag; 
        if &tag == b"MATH" {
            offset = Some(table.offset as u64);
            break;
        }  
    }

    // Math Constants

    // if let Some(ofs) = offset {
    //     cursor.set_position(ofs);
    //     let math_table = MathHeaderTable::read(&mut cursor).unwrap();

    //     let cnst_ofs = math_table.math_constants as u64;
    //     cursor.set_position(ofs + cnst_ofs);
    //     let math_const = MathConstantTable::read_table(&mut cursor).unwrap();
    //     println!("{:#?}", math_const);
    // } else { 
    //     println!("No math table! :(");
    // }

    // Glyph Info

    if let Some(offset) = offset {
        cursor.set_position(offset);
        let math_table = MathHeaderTable::read(&mut cursor).unwrap();

        let glyph_offset = math_table.math_glyph_info as u64;
        cursor.set_position(offset + glyph_offset);
        let glyph_info = MathGlyphInfoTable::read(&mut cursor).unwrap();

        println!("{:#?}", glyph_info);

        // let italics_offset = glyph_info.math_italics_correction as u64;
        // cursor.set_position(offset + glyph_offset + italics_offset);
        // let italics = MathItalicsCorrectionTable::read(&mut cursor).unwrap();
        // println!("{:?}", italics);

        let accents_offset = glyph_info.math_top_accent as u64;
        cursor.set_position(offset + glyph_offset + accents_offset);
        let accents = MathTopAccentTable::read(&mut cursor).unwrap();
        println!("{:?}", accents);
    }
}  