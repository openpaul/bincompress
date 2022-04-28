use std::io::{BufReader};
use log::{info, trace, warn};
use std::path::{Path};
use std::fs;
use flate2::read::GzDecoder;
use std::fs::File;
use crate::base;
pub use base::Row;
use csv::Reader;
use std::collections::HashMap;
use bio::io::fasta;

fn table2list(table: &str) -> HashMap<String,Vec<String>>{
    // HashMap storing the association between contigs
    // and bins
    let mut hm: HashMap<String,Vec<String>> = HashMap::new();

    // open gzipped csv file
    let file = File::open(table).unwrap();
    let reader = BufReader::new(GzDecoder::new(file));
    let mut rdr = Reader::from_reader(reader);
    // iterate csv
    for result in rdr.records() {

        // extract csv fields
        let record = result.unwrap();
        let contig = record.get(1).unwrap().to_string();
        let bin = record.get(2).unwrap();
        let binner = record.get(0).unwrap();

        // construct rel path of bin
        let p = Path::new(binner).join(bin.to_string().clone()).to_str().unwrap().to_string();

        // create vector if not exists in HashMap
        if !hm.contains_key(&contig){
            hm.insert(contig.clone(), Vec::new());
        }
        hm.get_mut(&contig).unwrap().push(p.clone());
    }
    return hm
}


fn create_outfolder(folder: &str, bin: &str){
    let path = Path::new(folder).join(bin);
    let prefix = path.parent().unwrap();
    if !prefix.exists(){
        log::info!("Creating dirs");
        fs::create_dir_all(prefix).expect("Could not create output folder");
    }
}

pub fn decompress(table: &str, outfolder: &str, assembly: &str){
    log::info!("Restoring bins");
    // define a HashMap [contig] --> [(binner, bin), (binner, bin)]
    // For fast iteration over assembly
    let contigs = table2list(table);

    // open all bin output files
    // Use a HashMap to have [(binner, bin)] --> output file
    let mut bins = HashMap::new();
    for (contig, inbins) in &contigs {
        for bin in inbins.iter(){
            if !bins.contains_key(bin){
                create_outfolder(outfolder, bin);
                let path = Path::new(outfolder).join(bin);
                let display = path.display();

                let mut file = match File::create(&path) {
                    Err(why) => panic!("couldn't open {}: {}", display, why),
                    Ok(file) => file,
                };
                let wrtr = fasta::Writer::new(file);
                bins.insert(bin.to_string(), wrtr);
            }
        }
    }

    // iterate assembly and fetch suitable output file name from 
    let reader = fasta::Reader::from_file(assembly).unwrap();
    for result in reader.records() {
        let record = result.expect("Error during fasta record parsing");
        if !contigs.contains_key(record.id()){
            continue
        }

        let bns =  contigs.get(record.id()).unwrap();
        for bn in bns.iter(){
            bins.get_mut(bn)
                .expect("Could not get bin fasta writer")
                .write_record(&record)
                .ok()
                .expect("Could not write record");
        }

    }

    // could have a hashsum check implementation as well
    // That would be a nice addition

}
