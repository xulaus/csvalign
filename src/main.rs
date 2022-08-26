use clap::Parser;
use indicatif::ProgressBar;
use std::{collections::HashMap, error::Error, fs::File};

/// Align several csv by column name and concatenate them
#[derive(Parser, Debug)]
struct Args {
    /// Only use columns that are in at least this many files
    #[clap(default_value_t = 1, long)]
    at_least: usize,
    input_files: Vec<String>,
    #[clap(short, long)]
    out: String,
    #[clap(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let headers = {
        let mut headers: HashMap<String, usize> = HashMap::new();
        for file in args.input_files.iter() {
            let mut reader = csv::Reader::from_reader(File::open(file)?);
            for header in reader.headers()? {
                if let Some(count) = headers.get_mut(header) {
                    *count += 1
                } else {
                    headers.insert(header.to_owned(), 1);
                }
            }
        }
        if args.verbose {
            println!("Found {} columns", headers.len())
        }
        headers.retain(|_k, &mut v| v > args.at_least);
        let mut headers = headers.into_keys().collect::<Vec<_>>();
        headers.sort();
        if args.verbose && args.at_least > 1 {
            println!("{} columns left after filtering", headers.len())
        }
        headers
    };

    if headers.len() == 0 {
        if args.verbose {
            println!("Nothing to do, as csv would be empty");
        }
        return Ok(());
    }

    let mut writer = csv::Writer::from_writer(File::create("out.csv")?);
    writer.write_record(&headers)?;

    println!("Concatinating {} files", args.input_files.len());
    let pb = ProgressBar::new(args.input_files.len() as u64);
    for file in args.input_files.iter() {
        let mut reader = csv::Reader::from_reader(File::open(file)?);
        let this_headers = reader.headers()?;
        let header_locations: Vec<_> = {
            headers
                .iter()
                .map(|header| this_headers.iter().position(|x| x == header))
                .collect()
        };
        for file_row in reader.records() {
            let row = file_row?;
            for field in header_locations.iter() {
                if let Some(loc) = field {
                    writer.write_field(row.get(*loc).unwrap())?;
                } else {
                    writer.write_field("")?;
                }
            }
            writer.write_record::<&[&str], _>(&[])?;
        }
        pb.inc(1);
    }
    pb.finish_and_clear();
    println!("Done!");

    Ok(())
}
