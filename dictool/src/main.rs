use anyhow::{Context, Result};
use clap::Parser;
use pmd_dic::KandFile;
use std::{fs::File, io::{Seek, SeekFrom, Write}, path::PathBuf};
/// dictool is a library used to read/write .dic file from Pokemon Super Mystery Dungeon (and maybe Gates To Infinity)
#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    /// Read a .dic file, and form a resulting json file
    Decode(DecodeParameter),
    /// Read a json file, and write the assorted .dic file
    Encode(EncodeParameter),
}

#[derive(Parser)]
pub struct DecodeParameter {
    /// the input .dic file
    input: PathBuf,
    /// the output .json files
    output: PathBuf,
}

#[derive(Parser)]
pub struct EncodeParameter {
    /// the input .json file
    input: PathBuf,
    /// the output .dic file
    output: PathBuf,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Decode(decode_parameter) => {
            decode(decode_parameter).context("can't decode the dic file")?
        }
        SubCommand::Encode(encode_parameter) => {
            encode(encode_parameter).context("can't encode the dic file")?;
        }
    };
    Ok(())
}

fn decode(decode_parameter: DecodeParameter) -> Result<()> {
    println!(
        "decoding the {:?} dic file to {:?}",
        decode_parameter.input, decode_parameter.output
    );
    let mut input = File::open(&decode_parameter.input)?;
    let kand = KandFile::new_from_reader(&mut input)?;
    let json_content = serde_json::to_vec_pretty(&kand)?;
    let mut output = File::create(decode_parameter.output)?;
    output.write_all(&json_content)?;
    let current_offset = output.seek(SeekFrom::Current(0))?;
    let last_offset = output.seek(SeekFrom::End(0))?;
    if current_offset != last_offset {
        println!("warning: there are unread bytes");
    };
    println!("done");
    Ok(())
}

fn encode(encode_parameter: EncodeParameter) -> Result<()> {
    println!(
        "encoding the {:?} file to the dic file {:?}",
        encode_parameter.input, encode_parameter.output
    );
    let input = File::open(&encode_parameter.input)?;
    let kand: KandFile = serde_json::from_reader(&input)?;
    let mut output = File::create(encode_parameter.output)?;
    kand.write(&mut output)?;
    println!("done");
    Ok(())
}
