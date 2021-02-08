use binread::BinReaderExt;
use binread::{derive_binread, BinRead};
use binwrite::BinWrite;
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek, Write};
//TODO: make serde an option

//size: 20 bytes
#[derive(BinRead, BinWrite)]
#[br(little)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KandChar {
    pub char: u16, //TODO: maybe check the file encoding and transform to a char
    pub start_x: u16,
    pub start_y: u16,
    pub glyth_width: u16,
    pub glyth_height: u16,
    pub unk1: u16,
    pub unk2: u16,
    pub unk3: u16,
    pub unk4: u16,
    pub unk5: u16,
}

#[derive_binread]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[br(little, magic = b"KAND")]
//#[br(assert(final_byte == 0, DecodeError::LastFourByteIsnt0(final_byte)))]
pub struct KandFile {
    pub unk1: u32,
    #[allow(dead_code)]
    #[br(temp)]
    char_number: u32,
    pub unk2: u32,
    #[br(count = char_number)]
    pub chars: Vec<KandChar>,
} //TODO: modify binwrite to handle magic and some other stuff

#[derive(BinWrite)]
struct KandFileWriter {
    magic: [u8; 4],
    unk1: u32,
    char_number: u32,
    unk2: u32,
    chars: Vec<KandChar>,
}

impl KandFileWriter {
    fn from_kandwriter(kand: KandFile) -> KandFileWriter {
        KandFileWriter {
            magic: b"KAND".clone(),
            unk1: kand.unk1,
            char_number: kand.chars.len() as u32,
            unk2: kand.unk2,
            chars: kand.chars,
        }
    }
}

impl KandFile {
    pub fn new_from_reader(reader: &mut (impl Read + Seek)) -> Result<Self, binread::Error> {
        Ok(reader.read_le()?)
    }

    pub fn write(self, writer: &mut impl Write) -> std::io::Result<()> {
        let kand_file_writer = KandFileWriter::from_kandwriter(self);
        kand_file_writer.write(writer)?;
        Ok(())
    }
}
