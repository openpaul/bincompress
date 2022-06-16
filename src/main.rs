use clap::{Args, Parser, Subcommand};
//use log::{info, trace, warn};
use camino::Utf8PathBuf;
use simple_logger::SimpleLogger;
mod compress;
mod restore;

mod lib;
pub use lib::Binner;

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
    append: bool,
}

fn main() {
    // setup logger
    let cli = Cli::parse();
    SimpleLogger::new().with_colors(true).init().unwrap();

    // Execute subcommand
    match &cli.command {
        Commands::Compress(args) => {
            log::info!("Compressing bins");
            compress::compress(&args.folder, &args.append, &args.output);
        }
        Commands::Restore(args) => {
            log::info!("Restoring compressed bins");
            restore::decompress(&args.table, &args.output, &args.assembly);
        }
    }
}
