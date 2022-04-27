use log::{info, trace, warn};
use std::fs;
use std::path::PathBuf;
use std::path::Path;
use bio::io::fasta;
use std::io::{BufWriter, Write};
use flate2::write;
use flate2::Compression;
use std::ffi::OsStr;
use std::fs::File;
use crate::base;
pub use base::Row;

fn get_fasta_ids(file: PathBuf) -> Vec<String>{
    let mut ids: Vec<String> = Vec::new();
    //let file = file.into_os_string().into_string().unwrap();
    let reader = fasta::Reader::from_file(file).unwrap();

    for result in reader.records() {
        let record = result.expect("Error during fasta record parsing");
        ids.push(record.id().to_string().clone());
    }
    ids
}


pub fn folder2list(folder: &str) -> Vec<Row> {
    let mut r: Vec<Row> = Vec::new();
    // accepts a folder 
    // returns a iterator (foldername, bin, contig)
    let paths = match fs::read_dir(folder){
        Ok(files) => files,
        Err(_) => panic!("Cant open folder {}", folder)
    };

    let name: String = Path::new(folder).file_name().unwrap().to_str().unwrap().into();
    for path in paths {
        let p = path.unwrap();
        // This cant be the best way to get the folder name
        //let name: String = p.path().parent().unwrap().file_name().unwrap().to_str().unwrap().into();
        let filename: String = p.file_name().into_string().unwrap();
        let ids: Vec<String> = get_fasta_ids(p.path());
        for id in ids.iter(){
            r.push(Row{contig: id.clone(), bin: filename.clone(), binner: name.clone()})
        }
    }
    r
}


pub fn writer(filename: &str, append: bool) -> Box<dyn Write> {
    let path = Path::new(filename);
    // opejn file object in append or new mode
    let file = match append {
        true =>match File::options().append(append).open(path) {
                Err(why) => panic!("couldn't open {}: {:?}", path.display(), why),
                Ok(file) => file,
                },
        false => match File::create(&path){
                Err(why) => panic!("couldn't open {}: {:?}", path.display(), why),
                Ok(file) => file,
                },
    };

    if path.extension() == Some(OsStr::new("gz")) {
        // Error is here: Created file isn't gzip-compressed
        Box::new(BufWriter::with_capacity(
            128 * 1024,
            write::GzEncoder::new(file, Compression::default()),
        ))
    } else {
        Box::new(BufWriter::with_capacity(128 * 1024, file))
    }
}
pub fn write_to_file(outfile: &str, values: &Vec<Vec<Row>>, append: &bool){
    let mut ap: bool = *append;
    if *append == true && Path::new(outfile).exists() == false {
        ap = false;
    }
    let mut writer_file = writer(outfile, ap);
    for contigs in values.iter(){
        for v in contigs.iter() {
            writer_file.write(v.test().as_bytes()).unwrap();
        }
    }
}
