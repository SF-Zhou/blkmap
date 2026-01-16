//! CLI tool for querying file extent maps (FIEMAP).
//!
//! Usage: blkmap PATH [--offset OFFSET] [--length LENGTH]

use blkmap::{Error, Fiemap};
use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

/// Query file physical extents (FIEMAP) for a given range on disk.
#[derive(Parser, Debug)]
#[command(name = "blkmap")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the file to query
    path: PathBuf,

    /// Starting offset in bytes (default: 0)
    #[arg(short, long, default_value = "0")]
    offset: u64,

    /// Length in bytes to query (default: entire file)
    #[arg(short, long)]
    length: Option<u64>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let result = if let Some(length) = args.length {
        args.path.fiemap_range(args.offset, length)
    } else if args.offset > 0 {
        // If offset is specified but length is not, query from offset to end
        args.path
            .fiemap_range(args.offset, u64::MAX.saturating_sub(args.offset))
    } else {
        args.path.fiemap()
    };

    match result {
        Ok(extents) => {
            if extents.is_empty() {
                println!("No extents found (file may be empty or use delayed allocation)");
            } else {
                println!(
                    "{:<6} {:<18} {:<18} {:<18} Flags",
                    "Index", "Logical", "Physical", "Length"
                );
                println!("{}", "-".repeat(80));

                for (i, extent) in extents.iter().enumerate() {
                    println!(
                        "{:<6} {:#018x} {:#018x} {:#018x} {:?}",
                        i, extent.logical, extent.physical, extent.length, extent.flags
                    );
                }

                println!("{}", "-".repeat(80));
                println!("Total: {} extent(s)", extents.len());
            }
            ExitCode::SUCCESS
        }
        Err(Error::NotSupported) => {
            eprintln!("Error: FIEMAP is not supported on this filesystem");
            ExitCode::from(2)
        }
        Err(Error::Io(e)) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
        Err(Error::InvalidRange { offset, length }) => {
            eprintln!(
                "Error: Invalid range (offset={}, length={})",
                offset, length
            );
            ExitCode::FAILURE
        }
    }
}
