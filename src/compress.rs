use bio::io::fasta;
use camino::{Utf8Path, Utf8PathBuf};
use flate2::write;
use flate2::Compression;
use rayon::prelude::*;
use serde_json;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::str;

use crate::lib;
pub use lib::Bin;
pub use lib::Binner;
pub use lib::{checksum256, get_width};

fn get_fasta_ids(file: &Utf8PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let reader = fasta::Reader::from_file(file)?;
    let ids: Vec<String> = reader
        .records()
        .map(|x| {
            x.expect("Error during fasta record parsing")
                .id()
                .to_string()
                .clone()
        })
        .collect();
    Ok(ids)
}

fn compress_bin(path: &Utf8PathBuf, binner_name: &String) -> Result<Bin, Box<dyn Error>> {
    let width = get_width(path).unwrap();
    log::info!("Building checksum");
    let hex = checksum256(path).unwrap();

    let filename: String = path.file_name().unwrap().into();
    let ids: Vec<String> = get_fasta_ids(path).unwrap();
    let b = Bin {
        name: filename.clone(),
        checksum: hex,
        contigs: ids,
        binner: binner_name.clone(),
        width: width,
    };
    return Ok(b);
}

fn bins_from_folder(folder: &Utf8PathBuf) -> Result<Binner, Box<dyn Error>> {
    let path = Utf8Path::new(folder);
    let binner_name: String = path.file_name().unwrap().into();
    let bin_paths: Vec<Utf8PathBuf> = fs::read_dir(folder)
        .expect("")
        .into_iter()
        .map(|x| Utf8PathBuf::from_path_buf(x.unwrap().path()).unwrap())
        .collect();
    let bins: Vec<Bin> = bin_paths
        .par_iter()
        .map(|x| compress_bin(&x, &binner_name).unwrap())
        .collect();

    let binner = Binner {
        name: binner_name.clone(),
        bins: bins,
    };

    return Ok(binner);
}

pub fn write_json(outfile: &str, values: Vec<Binner>, append: &bool) {
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
        true => match File::options().append(append).open(path) {
            Err(why) => panic!("couldn't open {}: {:?}", path, why),
            Ok(file) => file,
        },
        false => match File::create(&path) {
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

pub fn compress(folder: &Vec<String>, append: &bool, output: &str) {
    // create new binners object
    // in the future this could be populated with saved results
    let mut binners: Vec<Binner> = Vec::new();
    // add all bins for each bin folder
    for folder in folder.iter() {
        let f = Utf8PathBuf::from(folder);
        let new_bins = bins_from_folder(&f).unwrap();
        // check if we have the binner already saved, to not
        // have conflicts
        let mut add = true;
        for b in binners.iter() {
            if &b.name == &new_bins.name {
                add = false;
                log::warn!("Binner already in output, not adding again");
            }
        }
        if add {
            binners.push(new_bins);
        }
    }
    write_json(output, binners, append);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempdir::TempDir;

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
    fn t_fastaIDs() {
        let tmp_dir = TempDir::new("fasta").unwrap();
        let file_path = tmp_dir.path().join("binA.fasta");
        fs::write(&file_path, BIN_A).expect("Unable to write file");

        let ids = get_fasta_ids(&Utf8PathBuf::from_path_buf(file_path).unwrap())
            .expect("Could not get fasta ids");
        let expected = vec!["Contig_1", "Contig_2", "Contig_3"];
        assert_eq!(expected, ids);
    }

    #[test]
    fn t_bins() {
        let filename = "binA.fasta";
        let tmp_dir = TempDir::new("fasta").unwrap();
        let file_path = tmp_dir.path().join(filename);
        fs::write(&file_path, BIN_A).expect("Unable to write file");

        // complicated casting of path types
        let path = PathBuf::from(tmp_dir.path());
        let folder = Utf8PathBuf::from_path_buf(path).unwrap();
        let binner = bins_from_folder(&folder).expect("Could not get bins");
        let v = binner.bins[0].clone();
        let expected_ids = vec!["Contig_1", "Contig_2", "Contig_3"];

        assert_eq!(v.name, filename);
        assert_eq!(v.width, 20);
        assert_eq!(v.contigs, expected_ids);
    }
}
