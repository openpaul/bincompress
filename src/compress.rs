use log::{info, trace, warn};
use std::fs;
use std::str;
//use std::path::{PathBuf,Path};
use camino::{Utf8PathBuf,Utf8Path};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufWriter, Write,BufReader};
use std::io::BufRead;
use std::io;
use bio::io::fasta;
use flate2::write;
use flate2::Compression;
//use serde::{Serialize, Deserialize};
use serde_json;
//use checksums::{hash_file,Algorithm};


use crate::base;
pub use base::Binner;
pub use base::Bin;
pub use base::{checksum256,get_width};


fn get_fasta_ids(file: Utf8PathBuf) -> Vec<String>{
    let mut ids: Vec<String> = Vec::new();
    //let file = file.into_os_string().into_string().unwrap();
    let reader = fasta::Reader::from_file(file).unwrap();

    for result in reader.records() {
        let record = result.expect("Error during fasta record parsing");
        ids.push(record.id().to_string().clone());
    }
    ids
}

fn get_bins(folder: &str) -> fs::ReadDir {
    let paths = match fs::read_dir(folder){
        Ok(files) => files,
        Err(_) => panic!("Cant open folder {}", folder)
    };
    return paths
}


pub fn bins_from_folder(folder: &str) -> Binner {
    let bins: Vec<Bin> = Vec::new();
    let path = Utf8Path::new(folder);
    let binner_name: String = path.file_name().unwrap().into();
    let mut binner = Binner{name : binner_name.clone(),
                    bins: bins};

    for bin in get_bins(folder){

        let p = Utf8PathBuf::from_path_buf(bin.unwrap().path()).unwrap();

        let width = get_width(&p).unwrap();
        log::info!("Building checksum");
        let hex = checksum256(&p).unwrap();

        let filename: String = p.file_name().unwrap().into();
        let ids: Vec<String> = get_fasta_ids(p);
        let b = Bin{
            name: filename.clone(),
            checksum: hex,
            contigs: ids,
            binner: binner_name.clone(),
            width: width
        };
        binner.bins.push(b);
    }
    return binner
}

pub fn add_bins(parent: Vec<Bin>, add: &Vec<Bin>) -> Vec<Bin> {
    let mut parent_keys: Vec<String> = Vec::new();
    for bin in parent.iter() {
        parent_keys.push(bin.id());
    }
    //add.retain(|&x| !parent_keys.contains(&x.id()));

    return parent
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
