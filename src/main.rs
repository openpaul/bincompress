use clap::{Parser, Subcommand,Args};
use log::{info, trace, warn};
use simple_logger::SimpleLogger;
mod restore;
mod compress;

mod base;
pub use base::Row;

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
    #[clap(short, long,default_value_t = String::from("bins.csv.gz"))]
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
            let mut bins: Vec<Vec<Row>> = Vec::new();
            for folder in args.folder.iter(){
                let contigs: Vec<Row> = compress::folder2list(&folder);
                bins.push(contigs);
            }
            compress::write_to_file(&args.output, &bins, &args.append);
        }
    }
}
