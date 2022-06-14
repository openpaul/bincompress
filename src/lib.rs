use std::fs::File;
use camino::{Utf8PathBuf};
use std::io::{BufReader};
use std::io::BufRead;
use std::io::Error;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};


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


/// Function to compute sha256 hash on a file
///
/// Returns a result type with the hex digest of the 
/// sha256 hash.
///
/// Arguments:
/// * `path`: UTF8PathBuf pointing to the input
pub fn checksum256(path: &Utf8PathBuf) -> Result<String,Error> {
    let mut file = File::open(path)?;
    let mut sha256 = Sha256::new();
    std::io::copy(&mut file, &mut sha256)?;
    let hex: String = format!("{:X}", sha256.finalize());
    return Ok(hex)
}

/// Compute line-length (width) of FASTA file
///
/// Computes the width of a given FASTA file as a usize.
/// It does not validate the input. Output is only valid
/// for valid FASTA files.
///
/// Arguments:
/// * `path`: UTF8PathBuf pointing to the input
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
        checksum256(&file_path).unwrap();
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
