mod header;

use crate::elf::header::{Header, Ident};
use std::error;
use std::fs;
use std::path::PathBuf;

pub fn load(file: PathBuf) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let file = fs::read(file)?;

    let ident: Ident = <&[u8] as TryInto<&[u8; 0x10]>>::try_into(&file[..0x10])?.into();

    println!("{ident}");

    let header: Header = (file[..ident.class() as usize].as_ref(), ident).into();

    println!("header = {header:?}");

    println!("header = {}", header[0x00]);

    Ok(file)
}