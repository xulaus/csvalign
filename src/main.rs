use clap::Parser;
use std::error::Error;

mod align;
use align::{csvalign, AlignArgs};

mod uniq;
use uniq::{csvmakeuniq, MakeUniqArgs};

#[derive(Parser)]
enum Args {
    Align(AlignArgs),
    MakeUnique(MakeUniqArgs),
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args {
        Args::Align(args) => return csvalign(args),
        Args::MakeUnique(args) => return csvmakeuniq(args),
    };
}
