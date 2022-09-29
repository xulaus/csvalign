use clap::Parser;
use indicatif::ProgressBar;
use std::{collections::BTreeMap, error::Error, fs::File, io};

/// Align several csv files by column name and concatenate them
#[derive(Parser, Debug)]
pub struct AlignArgs {
    /// Only use columns that are in at least this many files
    #[arg(default_value_t = 1, long)]
    at_least: usize,
    input_files: Vec<String>,
    /// File to output to, if not specified standard out will be used
    #[arg(short, long)]
    out: Option<String>,
    #[arg(short, long)]
    verbose: bool,
}

pub fn csvalign(args: AlignArgs) -> Result<(), Box<dyn Error>> {
    let headers = {
        let mut headers: BTreeMap<String, usize> = BTreeMap::new();
        for file in args.input_files.iter() {
            let mut reader = csv::Reader::from_path(file)?;
            for header in reader.headers()? {
                let header = header.trim();
                if let Some(count) = headers.get_mut(header) {
                    *count += 1
                } else {
                    headers.insert(header.to_owned(), 1);
                }
            }
        }
        if args.verbose {
            eprintln!("Found {} columns", headers.len())
        }
        headers.retain(|_k, &mut v| v > args.at_least);
        let mut headers = headers.into_keys().collect::<Vec<_>>();
        headers.sort();
        if args.verbose && args.at_least > 1 {
            eprintln!("{} columns left after filtering", headers.len())
        }
        headers
    };

    if headers.len() == 0 {
        if args.verbose {
            eprintln!("Nothing to do, as csv would be empty");
        }
        return Ok(());
    }

    let mut writer: csv::Writer<Box<dyn io::Write>> = csv::Writer::from_writer(match args.out {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    });
    writer.write_record(&headers)?;

    eprintln!("Concatinating {} files", args.input_files.len());
    let pb = ProgressBar::new(args.input_files.len() as u64);
    for file in args.input_files.iter() {
        let mut reader = csv::Reader::from_path(file)?;
        let this_headers = reader.headers()?;
        let header_locations: Vec<_> = {
            headers
                .iter()
                .map(|header| this_headers.iter().position(|x| x.trim() == header))
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
    eprintln!("Done!");

    Ok(())
}
