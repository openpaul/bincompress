//use log::{info, trace, warn};
use std::fs;
use std::str;
use camino::{Utf8PathBuf,Utf8Path};
use std::fs::File;
use std::io::{BufWriter, Write};
use bio::io::fasta;
use flate2::write;
use flate2::Compression;
use serde_json;
use std::error::Error;


use crate::base;
pub use base::Binner;
pub use base::Bin;
pub use base::{checksum256, get_width};


fn get_fasta_ids(file: &Utf8PathBuf) -> Result<Vec<String>, Box<dyn Error>>{
    let mut ids: Vec<String> = Vec::new();
    //let file = file.into_os_string().into_string().unwrap();
    let reader = fasta::Reader::from_file(file)?;

    for result in reader.records() {
        let record = result.expect("Error during fasta record parsing");
        ids.push(record.id().to_string().clone());
    }
    Ok(ids)
}

fn get_bins(folder: &str) -> Result<fs::ReadDir,Box<dyn Error>>  {
    Ok(fs::read_dir(folder)?)
}


pub fn bins_from_folder(folder: &str) -> Result<Binner,Box<dyn Error>> {
    let bins: Vec<Bin> = Vec::new();
    let path = Utf8Path::new(folder);
    let binner_name: String = path.file_name().unwrap().into();
    let mut binner = Binner{name : binner_name.clone(),
                            bins: bins};

    for bin in get_bins(folder)
            .expect("Could not get bins for folder"){

        let p = Utf8PathBuf::from_path_buf(bin.unwrap().path()).unwrap();

        let width = get_width(&p).unwrap();
        log::info!("Building checksum");
        let hex = checksum256(&p).unwrap();

        let filename: String = p.file_name().unwrap().into();
        let ids: Vec<String> = get_fasta_ids(&p).unwrap();
        let b = Bin{
            name: filename.clone(),
            checksum: hex,
            contigs: ids,
            binner: binner_name.clone(),
            width: width
        };
        binner.bins.push(b);
    }
    return Ok(binner)
}


pub fn write_json(outfile: &str, values: Vec<Binner>, append: &bool){
    let mut ap: bool = *append;
    if *append == true && Utf8Path::new(outfile).exists() == false {
        ap = false;
    }
    let writer_file = writer(outfile, ap);
    serde_json::to_writer(writer_file, &values).unwrap();
}


pub fn writer(filename: &str, append: bool) -> Box<dyn Write> {
    let path = Utf8Path::new(filename);
    // open file object in append or new mode
    let file = match append {
        true =>match File::options().append(append).open(path) {
                Err(why) => panic!("couldn't open {}: {:?}", path, why),
                Ok(file) => file,
                },
        false => match File::create(&path){
                Err(why) => panic!("couldn't open {}: {:?}", path, why),
                Ok(file) => file,
                },
    };

    if path.extension() == Some("gz") {
        // Error is here: Created file isn't gzip-compressed
        Box::new(BufWriter::with_capacity(
            128 * 1024,
            write::GzEncoder::new(file, Compression::default()),
        ))
    } else {
        Box::new(BufWriter::with_capacity(128 * 1024, file))
    }
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
    fn t_fastaIDs(){
        let tmp_dir = TempDir::new("fasta").unwrap();
        let file_path = tmp_dir.path().join("binA.fasta");
        fs::write(&file_path, BIN_A).expect("Unable to write file"); 

        let ids = get_fasta_ids(&Utf8PathBuf::from_path_buf(file_path).unwrap())
                .expect("Could not get fasta ids");
        let expected = vec!["Contig_1", "Contig_2", "Contig_3"];
        assert_eq!(expected, ids);
    }

}
