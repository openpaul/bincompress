use std::fs::File;
use std::io;
use camino::{Utf8PathBuf,Utf8Path};
use std::io::{BufWriter, Write,BufReader};
use std::io::BufRead;
use std::io::Error;
use sha2::{Sha256, Sha512, Digest};
use serde::{Deserialize, Serialize};
#[derive(Debug)]
pub struct Row {
    pub contig: String,
    pub bin: String,
    pub binner: String
}
impl Row{
    pub fn test(&self)-> String{
        format!("{},{},{}\n", self.binner, self.contig, self.bin)
    }

}
#[derive(Debug, Deserialize, Serialize)]
pub struct folders {
    pub name: String,
    pub bins: Vec<Bin>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Binner {
    pub name: String,
    pub bins: Vec<Bin>
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Bin {
    pub name: String,
    pub binner: String,
    pub checksum: String,
    pub contigs: Vec<String>,
    pub width: usize,
}

impl Bin {
    pub fn id(&self) -> String {
        format!("{}/{}",self.binner, self.name)
    }
}

pub fn checksum256(path: &Utf8PathBuf) -> Result<String,Error> {
    let mut file = File::open(path)?;
    let mut sha256 = Sha256::new();
    io::copy(&mut file, &mut sha256)?;
    let hex: String = format!("{:X}", sha256.finalize());
    return Ok(hex)
}

pub fn get_width(path: &Utf8PathBuf) -> Result<usize,Error> {
    let mut width = 0;
    let file = File::open(path)?;
    let r = BufReader::new(file);
    for line in r.lines() {
        let l = line?;
        if l.starts_with(">"){
            continue
        }
        let i = l.len();
        if i > width {width = i}
    }
    Ok(width)
}
