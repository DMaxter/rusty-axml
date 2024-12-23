use std::fmt;
use std::io::{
    Error,
    Cursor,
};
use byteorder::{
    LittleEndian,
    ReadBytesExt
};

/* Type identifiers for chunks. Only includes the ones related to XML */
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ChunkType {
    ResNullType                 = 0x0000,
    ResStringPoolType           = 0x0001,
    ResTableType                = 0x0002,
    ResXmlType                  = 0x0003,

    /* Chunk types in RES_XML_Type */
    // TODO: for some reason this chunk has the same value has ResXmlStartNamespaceType which is
    // annoying. Need to figure out a way to deal with this. In the meantime, ignore it.
    // ResXmlFirstChunkType     = 0x0100,
    ResXmlStartNamespaceType    = 0x0100,
    ResXmlEndNamespaceType      = 0x0101,
    ResXmlStartElementType      = 0x0102,
    ResXmlEndElementType        = 0x0103,
    ResXmlCDataType             = 0x0104,
    ResXmlLastChunkType         = 0x017f,

    /* This contains a uint32_t array mapping strings in the string
     * pool back to resource identifiers.  It is optional. */
    ResXmlResourceMapType       = 0x0180,

    /* Chunk types in RES_TABLE_Type */
    ResTablePackageType         = 0x0200,
    ResTableTypeType            = 0x0201,
    ResTableTypeSpecType        = 0x0202,
    ResTableLibraryType         = 0x0203
}

impl ChunkType {
    pub fn parse_block_type(buff: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let raw_block_type = buff.read_u16::<LittleEndian>();
        let raw_block_type = match raw_block_type {
            Ok(block) => block,
            Err(e) => return Err(e),
        };

        let block_type = match raw_block_type {
            0x0000 => ChunkType::ResNullType,
            0x0001 => ChunkType::ResStringPoolType,
            0x0002 => ChunkType::ResTableType,
            0x0003 => ChunkType::ResXmlType,

            /* Chunk types in RES_XML_TYPE */
            // TODO: see comment above.
            // 0x0100 => ChunkType::ResXmlFirstChunkType,
            0x0100 => ChunkType::ResXmlStartNamespaceType,
            0x0101 => ChunkType::ResXmlEndNamespaceType,
            0x0102 => ChunkType::ResXmlStartElementType,
            0x0103 => ChunkType::ResXmlEndElementType,
            0x0104 => ChunkType::ResXmlCDataType,
            0x017f => ChunkType::ResXmlLastChunkType,

            /* This contains a uint32_t array mapping strings in the string
             * pool back to resource identifiers. It is optional. */
            0x0180 => ChunkType::ResXmlResourceMapType,

            /* Chunk types in RES_TABLE_TYPE */
            0x0200 => ChunkType::ResTablePackageType,
            0x0201 => ChunkType::ResTableTypeType,
            0x0202 => ChunkType::ResTableTypeSpecType,
            0x0203 => ChunkType::ResTableLibraryType,

            /* If we find an unknown type, we stop and panic */
            _ => panic!("Error: unknown block type {:02X}", raw_block_type)
        };

        Ok(block_type)
    }
}

/* Implementation of the UpperHex trait for ChunkType */
impl fmt::UpperHex for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChunkType::ResNullType => write!(f, "{:X}", 0x0000),
            ChunkType::ResStringPoolType => write!(f, "{:X}", 0x0001),
            ChunkType::ResTableType => write!(f, "{:X}", 0x0002),
            ChunkType::ResXmlType => write!(f, "{:X}", 0x0003),

            // TODO: see comment above.
            // ChunkType::ResXmlFirstChunkType => write!(f, "{:X}", 0x0100),
            ChunkType::ResXmlStartNamespaceType => write!(f, "{:X}", 0x0100),
            ChunkType::ResXmlEndNamespaceType => write!(f, "{:X}", 0x0101),
            ChunkType::ResXmlStartElementType => write!(f, "{:X}", 0x0102),
            ChunkType::ResXmlEndElementType => write!(f, "{:X}", 0x0103),
            ChunkType::ResXmlCDataType => write!(f, "{:X}", 0x0104),
            ChunkType::ResXmlLastChunkType => write!(f, "{:X}", 0x017f),

            ChunkType::ResXmlResourceMapType => write!(f, "{:X}", 0x0180),

            ChunkType::ResTablePackageType => write!(f, "{:X}", 0x0200),
            ChunkType::ResTableTypeType => write!(f, "{:X}", 0x0201),
            ChunkType::ResTableTypeSpecType => write!(f, "{:X}", 0x0202),
            ChunkType::ResTableLibraryType => write!(f, "{:X}", 0x0203),
        }?;
        Ok(())
    }
}
