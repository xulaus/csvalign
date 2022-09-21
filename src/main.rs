use clap::Parser;
use std::error::Error;

mod align;
use align::{csvalign, AlignArgs};

mod dedup;
use dedup::{csvdedup, DedupArgs};

#[derive(Parser)]
enum Args {
    Align(AlignArgs),
    Dedup(DedupArgs),
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args {
        Args::Align(args) => return csvalign(args),
        Args::Dedup(args) => return csvdedup(args),
    };
}
