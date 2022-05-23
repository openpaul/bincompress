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

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;
    use std::fs;
    use std::io;

    const BIN_A: &str = ">Contig_1
AGCTACGACATACG
GTACGACT
AAAAAAAAAAAAAAAAAAAA
>Contig_2
AGGTTTTTTTGG
>Contig_3
AGATGACTAAAA
ATGGTTTTTTTTTTTT
";

    #[test]
    fn t_checksum256(){
        let tmp_dir = TempDir::new("fasta").unwrap();
        let file_path = tmp_dir.path().join("binA.fasta");
        fs::write(&file_path, BIN_A).expect("Unable to write file"); 

        let cs = checksum256(&Utf8PathBuf::from_path_buf(file_path).unwrap())
                .expect("Could not compute checksu:");
        let expected = "667910636B177485DF8DB1C426A9FA2A243003FD44022E7BB485791EA1081041";
        assert_eq!(expected, cs);
    }

    #[test]
    #[should_panic]
    fn t_checksum256_missing_file(){
        let file_path = Utf8PathBuf::from("missing.fasta");
        let cs = checksum256(&file_path).unwrap();
    }

    #[test]
    fn t_get_width(){
        let tmp_dir = TempDir::new("fasta").unwrap();
        let file_path = tmp_dir.path().join("binA.fasta");
        fs::write(&file_path, BIN_A).expect("Unable to write file"); 

        let cs = get_width(&Utf8PathBuf::from_path_buf(file_path).unwrap())
                .expect("Could not compute width");
        let expected = 20;
        assert_eq!(expected, cs);
    }
}
