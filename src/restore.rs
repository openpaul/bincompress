use std::fs;
use std::io::{BufReader};
use std::io::Error;
use std::fs::File;
use serde_json;
use camino::{Utf8Path};
use flate2::read::GzDecoder;
use std::collections::HashMap;
use bio::io::fasta;

use crate::lib;
pub use lib::Binner;
pub use lib::{checksum256,get_width};


fn create_outfolder(folder: &str, binner: &str) -> Result<bool, Box<Error>> {
    let path = Utf8Path::new(folder).join(binner);
    if !path.exists(){
        log::info!("Creating dirs");
        fs::create_dir_all(path).expect("Could not create output folder");
    }
    Ok(true)
}

fn binner_from_json(infile: &str) -> Result<Vec<Binner>, Box<Error>>{
    let file = File::open(infile)?;
    let reader = BufReader::new(GzDecoder::new(file));
    let json = serde_json::from_reader(reader);
    Ok(json.unwrap())
}

pub fn decompress(infile: &str, outfolder: &str, assembly: &str){
    log::info!("Restoring bins");

    let binners = binner_from_json(infile)
                    .expect("Could not load from JSON");

    for binner in binners.iter(){
        log::info!("Restoring bins: {}", &binner.name);
        create_outfolder(outfolder, &binner.name)
            .expect("Could not create output folder");

        // for each bin
        for bin in binner.bins.iter(){
            log::info!("Restoring bins: {}", &bin.name);

            let mut contigs = HashMap::new();

            let reader = fasta::Reader::from_file(assembly).unwrap();
            for result in reader.records() {
                let record = result.expect("Error during fasta record parsing");
                if bin.contigs.contains(&record.id().to_string()){
                    contigs.insert(record.id().to_string(), record);
                }
            }

            // now that all seq are in memory
            let path = Utf8Path::new(outfolder).join(&binner.name).join(&bin.name);

            let file = match File::create(&path) {
                Err(why) => panic!("couldn't open {}: {}", path, why),
                Ok(file) => file,
            };

            // write to fasta
            let mut wrtr = fasta::Writer::new(file);
            for contig in bin.contigs.iter(){
                if contigs.contains_key(contig){
                    wrtr.write_record_width(contigs.get(contig).unwrap(), bin.width)
                        .ok()
                        .expect("Could not write record");
                }
            }
            drop(wrtr); // closing file before checksum computation

            let checksum = checksum256(&path).unwrap();
            if checksum != bin.checksum {
                log::warn!("Restored file is not the same");
                panic!("Stopping");
            }


        }

    }
}
