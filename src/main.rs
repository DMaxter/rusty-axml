use std::{
    env,
    fs,
    process::exit,
};
use std::io::{
    Read,
    Error,
};
use byteorder::{
    LittleEndian,
    ReadBytesExt
};

struct XmlTypes {}

/* Type identifiers for chunks. Only includes the ones related to XML */
#[allow(dead_code)]
impl XmlTypes {
    pub const RES_NULL_TYPE: u16                = 0x0000;
    pub const RES_STRING_POOL_TYPE: u16         = 0x0001;
    pub const RES_TABLE_TYPE: u16               = 0x0002;
    pub const RES_XML_TYPE: u16                 = 0x0003;

    /* Chunk types in RES_XML_TYPE */
    pub const RES_XML_FIRST_CHUNK_TYPE: u16     = 0x0100;
    pub const RES_XML_START_NAMESPACE_TYPE: u16 = 0x0100;
    pub const RES_XML_END_NAMESPACE_TYPE: u16   = 0x0101;
    pub const RES_XML_START_ELEMENT_TYPE: u16   = 0x0102;
    pub const RES_XML_END_ELEMENT_TYPE: u16     = 0x0103;
    pub const RES_XML_CDATA_TYPE: u16           = 0x0104;
    pub const RES_XML_LAST_CHUNK_TYPE: u16      = 0x017f;

    /* This contains a uint32_t array mapping strings in the string
     * pool back to resource identifiers.  It is optional. */
    pub const RES_XML_RESOURCE_MAP_TYPE: u16    = 0x0180;

    /* Chunk types in RES_TABLE_TYPE */
    pub const RES_TABLE_PACKAGE_TYPE: u16       = 0x0200;
    pub const RES_TABLE_TYPE_TYPE: u16          = 0x0201;
    pub const RES_TABLE_TYPE_SPEC_TYPE: u16     = 0x0202;
    pub const RES_TABLE_LIBRARY_TYPE: u16       = 0x0203;
}

/* Header that appears at the beginning of every chunk */
struct ChunkHeader {
    /* Type identifier for this chunk.
     * The meaning of this value depends on the containing chunk. */
    chunk_type: u16,

    /* Size of the chunk header (in bytes).
     * Adding this value to the address of the chunk allows you to find
     * its associated data (if any). */
    header_size: u16,

    /* Total size of this chunk (in bytes).
     * This is the chunkSize plus the size of any data associated with the
     * chunk. Adding this value to the chunk allows you to completely skip
     * its contents (including any child chunks). If this value is the same
     * as chunkSize, there is no data associated with the chunk */
    size: u32,
}

impl ChunkHeader {

    fn from_buff(mut axml_buff: &[u8], expected_type: u16) -> Result<Self, Error> {
        /* Minimum size, for a chunk with no data */
        let minimum_size = 8;

        /* Get chunk type */
        let chunk_type = axml_buff.read_u16::<LittleEndian>().unwrap();

        /* Check if this is indeed of the expected type */
        if chunk_type != expected_type {
            panic!("Error: chunk is not an XML chunk");
        }

        /* Get chunk header size and total size */
        let chunk_header_size = axml_buff.read_u16::<LittleEndian>().unwrap();
        let chunk_total_size = axml_buff.read_u32::<LittleEndian>().unwrap();

        /* Exhaustive checks on the announced sizes */
        if chunk_header_size < minimum_size {
            panic!("Error: parsed header size is smaller than the minimum");
        }

        if chunk_total_size < minimum_size.into() {
            panic!("Error: parsed total size is smaller than the minimum");
        }

        if chunk_total_size < chunk_header_size.into() {
            panic!("Error: parsed total size if smaller than parsed header size");
        }

        /* Build and return the object */
        Ok(ChunkHeader {
            chunk_type: chunk_type,
            header_size: chunk_header_size,
            size: chunk_total_size,
        })
    }
}

/* Header of a chunk representing a pool of strings
 *
 * Definition for a pool of strings.  The data of this chunk is an
 * array of uint32_t providing indices into the pool, relative to
 * stringsStart.  At stringsStart are all of the UTF-16 strings
 * concatenated together; each starts with a uint16_t of the string's
 * length and each ends with a 0x0000 terminator.  If a string is >
 * 32767 characters, the high bit of the length is set meaning to take
 * those 15 bits as a high word and it will be followed by another
 * uint16_t containing the low word.
 *
 * If styleCount is not zero, then immediately following the array of
 * uint32_t indices into the string table is another array of indices
 * into a style table starting at stylesStart.  Each entry in the
 * style table is an array of ResStringPool_span structures.
 */
struct StringPoolHeader {
    /* Chunk header */
    header: ChunkHeader,

    /* Number of strings in this pool (number of uint32_t indices that
     * follow in the data). */
    string_count: u32,

     /* Number of style span arrays in the pool (number of uint32_t
      * indices follow the string indices). */
    style_count: u32,

    /* Flags. Can take two values:
     *      - SORTED_FLAG = 1<<0,
     *      - UTF8_FLAG = 1<<8
     *
     * If SORTED_FLAG is set, the string index is sorted by the string
     * values (based on strcmp16()).
     *
     * If UTF8_FLAG is set, the string pool is ended in UTF-8.  */
    flags: u32,

    /* Index from header of the string data. */
    strings_start: u32,

    /* Index from header of the style data. */
    styles_start: u32
}

impl StringPoolHeader{

    fn from_buff(mut axml_buff: &[u8]) -> Result<Self, Error> {
        /* Parse chunk header */
        let header = ChunkHeader::from_buff(axml_buff, XmlTypes::RES_STRING_POOL_TYPE)
                     .expect("Error: cannot get chunk header from string pool");

        /* Get remaining members */
        let string_count = axml_buff.read_u32::<LittleEndian>().unwrap();
        let style_count = axml_buff.read_u32::<LittleEndian>().unwrap();
        let flags = axml_buff.read_u32::<LittleEndian>().unwrap();
        let strings_start = axml_buff.read_u32::<LittleEndian>().unwrap();
        let styles_start = axml_buff.read_u32::<LittleEndian>().unwrap();

        /* Build and return the object */
        Ok(StringPoolHeader {
            header: header,
            string_count: string_count,
            style_count: style_count,
            flags: flags,
            strings_start: strings_start,
            styles_start: styles_start
        })
    }
}

fn get_next_block_type(mut axml_buff: &[u8]) -> Result<u16, Error> {
    let raw_block_type = axml_buff.read_u16::<LittleEndian>().unwrap();

    let block_type = match raw_block_type {
        0x0100 => XmlTypes::RES_XML_FIRST_CHUNK_TYPE,
        0x0100 => XmlTypes::RES_XML_START_NAMESPACE_TYPE,
        0x0101 => XmlTypes::RES_XML_END_NAMESPACE_TYPE,
        0x0102 => XmlTypes::RES_XML_START_ELEMENT_TYPE,
        0x0103 => XmlTypes::RES_XML_END_ELEMENT_TYPE,
        0x0104 => XmlTypes::RES_XML_CDATA_TYPE,
        0x017f => XmlTypes::RES_XML_LAST_CHUNK_TYPE,
        _ => XmlTypes::RES_NULL_TYPE
    };

    Ok(block_type)
}

fn main() {
    /* Check CLI arguments */
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./{:} [AXML]", args[0]);
        exit(22);
    }

    let axml_path = &args[1];
    println!("[+] Parsing {}", axml_path);

    let mut raw_file = fs::File::open(axml_path).expect("Error: cannot open AXML file");
    let mut axml_buff = Vec::new();
    raw_file.read_to_end(&mut axml_buff).expect("Error: cannot read AXML file");

    let header = ChunkHeader::from_buff(&axml_buff, XmlTypes::RES_XML_TYPE)
                 .expect("Error: cannot parse AXML header");

    println!("Header chunk_type: {:02X}", header.chunk_type);
    println!("Header header_size: {:02X}", header.header_size);
    println!("File size: {:02X}", header.size);
    println!("--------------------");

    let next_block_type = get_next_block_type(&axml_buff);
    let next_block_type = (&axml_buff).read_u16::<LittleEndian>().unwrap();
    read(&axml_buff[header.header_size as usize..]);
}

fn read(mut data: &[u8]) {
    println!("Type: {:02X}", data.read_u16::<LittleEndian>().unwrap());
    println!("Header size: {:02X}", data.read_u16::<LittleEndian>().unwrap());
    println!("Sizecount: {:02X}", data.read_u32::<LittleEndian>().unwrap());
    println!("Strings count: {:02X}", data.read_u32::<LittleEndian>().unwrap());
    println!("Styles count: {:02X}", data.read_u32::<LittleEndian>().unwrap());
    println!("Flags: {:02X}", data.read_u32::<LittleEndian>().unwrap());
    println!("Strings start: {:02X}", data.read_u32::<LittleEndian>().unwrap());
    println!("Styles start: {:02X}", data.read_u32::<LittleEndian>().unwrap());
}