# OpenType Fonts Math table reader

## Offset Table

If the font file contains only one font, the Offset Table will begin at byte 0 of the file. If the font file is a TrueType collection, the beginning point of the Offset Table for each font is indicated in the TTCHeader.  This should only happen when the file format has the extension `.ttc` however.  We will not recognize `ttc` file formats.

```
Type    Name           Description
------  ----------     ----------------------
ULONG   sfntVersion    0x00010000 or 'OTTO' — see below.
USHORT  numTables      Number of tables.
USHORT  searchRange    (Maximum power of 2 <= numTables) x 16.
USHORT  entrySelector  Log2(maximum power of 2 <= numTables).
USHORT  rangeShift     NumTables x 16-searchRange.
```

Total size: 16*4 + 32 = 16*6.

## Table Record

The Offset Table is followed immediately by the Table Record entries. Entries in the Table Record must be sorted in _ascending_ order by tag. Offset values in the Table Record are measured from the _start_ of the font file.


```
Type	Name	  Description
------  -------   -------------------------
ULONG   tag       4 -byte identifier.
ULONG   checkSum  CheckSum for this table.
ULONG   offset	  Offset from beginning of TrueType font file.
ULONG   length    Length of this table.
```

## Checksums

Checksums are calculated using the following C code:

```
ULONG
CalcTableChecksum(ULONG *Table, ULONG Length)
{
ULONG Sum = 0L;
ULONG *Endptr = Table+((Length+3) & ~3) / sizeof(ULONG);
while (Table < EndPtr)
    Sum += *Table++;
return Sum;
}
```

Note: This function implies that the length of a table must be a multiple of four bytes. In fact, a font is not considered structurally proper without the correct padding. All tables must begin on four byte boundries, and any remaining space between tables is padded with zeros. The length of all tables should be recorded in the table record with their actual length (not their padded length).

## OpenType tables

Determine which are needed

# Math tables

## Math Table Header

```
Type    Name             Description
------  --------------   -----------------------------
Fixed   Version          Version of the MATH table, currently set to 0x00010000
Offset  MathConstants    Offset to MathConstants table - from the beginning of MATH table.
Offset  MathGlyphInfo    Offset to MathGlyphInfo table - from the beginning of MATH table.
Offset  MathVariants     Offset to MathVariants table - from the beginning of MATH table.
```

## Math Constants Table

This table will contain many things called a `Math Value Record`.  This is defined by:

```
Type    Name          Description
------  ------------  ---------------------
SHORT   Value         The X or Y value in design units
Offset  DeviceTable   Offset to the device table – from the beginning of parent table. May be NULL. Suggested format for device table is 1.
```

The math constants table is kind of huge... Will need to document this.

## Math Glyph Info

TODO: How do you lookup a glyph from this?

This table contains position information defined on a per-glyph basis.

```
Type    Name                         Description
------  ---------------------------  -------------------------------------
Offset  MathItalicsCorrectionInfo    Offset to MathItalicsCorrectionInfo table - from the beginning of MathGlyphInfo table.
Offset  MathTopAccentAttachment      Offset to MathTopAccentAttachment table - from the beginning of MathGlyphInfo table.
Offset  ExtendedShapeCoverage        Offset to coverage table for Extended Shape glyphs - from the beginning of MathGlyphInfo table. When the left or right glyph of a box is an extended shape variant, the (ink) box (and not the default position defined by values in MathConstants table) should be used for vertical positioning purposes. May be NULL..
Offset  MathKernInfo                 Offset to MathKernInfo table - from the beginning of MathGlyphInfo table.
```

# Features and Lookups

To implement features, a client applies the lookups in the order the lookup definitions occur in the LookupList. As a result, within the GSUB or GPOS table, lookups from several different features may be interleaved during text processing. A lookup is finished when the client locates a target glyph or glyph context and performs a substitution (if specified) or a positioning (if specified).

Note: The substitution (GSUB) lookups always occur before the positioning (GPOS) lookups. The lookup sequencing mechanism in TrueType relies on the font to determine the proper order of text-processing operations.

## Feature List Table

The headers of the GSUB and GPOS tables contain offsets to Feature List tables (FeatureList) that enumerate all the features in a font. 

The Feature List Table
```
Type    Name                          Description
------  -----------------------       --------------------------------
USHORT  FeatureCount                  Number of FeatureRecords in this table
struct  FeatureRecord[FeatureCount]   Array of FeatureRecords-zero-based (first feature has FeatureIndex = 0)-listed alphabetically by FeatureTag
```

The Feature Record

```
Type    Name           Description
------  -------------  ----------------------------
Tag     FeatureTag     4-byte feature identification tag
Offset  Feature        Offset to Feature table-from beginning of FeatureList
```

## Feature Table

<https://www.microsoft.com/typography/otspec/chapter2.htm>

TODO:
[ ] Gracefully fail reading version numbers > 1.8.
[ ] 

## Steps

1. The TTC header yields the number of tables.
2. The following, the Offset Table -- beginning of TTF
