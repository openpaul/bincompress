use clap::{Parser, Subcommand,Args};
//use log::{info, trace, warn};
use simple_logger::SimpleLogger;
mod restore;
mod compress;

mod base;
pub use base::Binner;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compress bin folder to csv
    Compress(Compress),
    /// Restore bins from table and assembly
    Restore(Restore),
}
#[derive(Args)]
struct Restore {
    table: String,
    assembly: String,
    /// Path to output file,
    /// will create if not present or append
    #[clap(short, long,default_value_t = String::from("restore"))]
    output: String,
    #[clap(short, long)]
    verbose: bool,
}
#[derive(Args)]
struct Compress {
    /// Path to binfolder
    #[clap(required = true)]
    folder: Vec<String>,
    /// Path to output file,
    /// will create if not present or append
    #[clap(short, long,default_value_t = String::from("bins.json.gz"))]
    output: String,
    #[clap(short, long)]
    /// Append to exsting table
    append: bool
}

fn main() {
    // setup logger
    let cli = Cli::parse();
    SimpleLogger::new().with_colors(true).init().unwrap();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Restore(args) => {
            log::info!("Restoring compressed bins");
            restore::decompress(&args.table, &args.output, &args.assembly);
        }
        Commands::Compress(args) => {
            log::info!("Compressing bins");
            // create new binners object
            // in the future this could be populated with saved results
            let mut binners: Vec<Binner> = Vec::new();
            // add all bins for each bin folder
            for folder in args.folder.iter(){
                let new_bins = compress::bins_from_folder(&folder).unwrap();
                // check if we have the binner already saved, to not 
                // have conflicts
                let mut add = true;
                for b in binners.iter(){
                    if &b.name == &new_bins.name {
                        add = false;
                        log::warn!("Binner already in output, not adding again");
                    }
                }
                if add {
                    binners.push(new_bins);
                }
            }
            compress::write_json(&args.output, binners, &args.append);
        }
    }
}
