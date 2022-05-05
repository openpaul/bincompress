use std::io::{BufReader};
use log::{info, trace, warn};
use camino::{Utf8PathBuf,Utf8Path};
use std::fs;
use flate2::read::GzDecoder;
use std::fs::File;
use crate::base;
pub use base::Binner;
use csv::Reader;
use std::collections::HashMap;
use bio::io::fasta;
use serde_json;


fn create_outfolder(folder: &str, binner: &str){
    let path = Utf8Path::new(folder).join(binner);
    if !path.exists(){
        log::info!("Creating dirs");
        fs::create_dir_all(path).expect("Could not create output folder");
    }
}

fn binner_from_json(infile: &str) -> Vec<Binner>{
    let file = File::open(infile).unwrap();
    let reader = BufReader::new(GzDecoder::new(file));
    let json = serde_json::from_reader(reader);
    return json.unwrap()
}

pub fn decompress(infile: &str, outfolder: &str, assembly: &str){
    log::info!("Restoring bins");
    // define a HashMap [contig] --> [(binner, bin), (binner, bin)]
    // For fast iteration over assembly
    //let contigs = table2list(table);
    let binners = binner_from_json(infile);
    // iterate binners and create 
    // bins from saved information
    // To retain order of contigs
    // We need to either use random indexing
    // or iterate the assembly multiple times
    // We assume that no index is present, and we dont 
    // want to store DNA in memory
    // Thus we need to iterate several times
    for binner in binners.iter(){
        log::info!("Restoring bins: {}", &binner.name);
        create_outfolder(outfolder, &binner.name);

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
                    wrtr.write_record_width(contigs.get(contig).unwrap(), 50)
                        .ok()
                        .expect("Could not write record");
                }
            }
        }

    }
}
