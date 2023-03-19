use clap::Parser;
use color_eyre::eyre::{WrapErr,eyre, Result};
use std::{collections::BTreeMap, fs::File, io};

#[derive(Debug)]
enum UniqType {
    First,
    Max(usize),
    Min(usize),
}

/// Take a single csv file and make one of the columns unique
#[derive(Parser, Debug)]
pub struct DedupArgs {
    input_file: String,
    /// Column to ensure every entry is unique in
    unique_column: String,

    /// Choose which entry to keep by keeping the maximum value in this column
    #[arg(long, group = "select")]
    max_by: Option<String>,

    /// Choose which entry to keep by keeping the minimum value in this column
    #[arg(long, group = "select")]
    min_by: Option<String>,

    /// File to output to, if not specified standard out will be used
    #[arg(short, long)]
    out: Option<String>,
}

pub fn csvdedup(args: DedupArgs) -> Result<()> {
    let mut reader = csv::Reader::from_path(&args.input_file)?;
    let mut writer: csv::Writer<Box<dyn io::Write>> = csv::Writer::from_writer(match args.out {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    });
    let this_headers = reader.headers()?;
    let unique_col = this_headers
        .iter()
        .position(|x| x.trim() == args.unique_column.trim())
        .ok_or(eyre!("Couldn't find column '{}' in input csv", args.unique_column.trim()))?;

    let select_type = if let Some(col) = args.max_by {
        let pos = this_headers
            .iter()
            .position(|x| x.trim() == col)
            .ok_or(eyre!("Couldn't find max-by column'{}' in input csv", col.trim()))?;
        UniqType::Max(pos)
    } else if let Some(col) = args.min_by {
        let pos = this_headers
            .iter()
            .position(|x| x.trim() == col)
            .ok_or(eyre!("Couldn't find min-by column'{}' in input csv", col.trim()))?;
        UniqType::Min(pos)
    } else {
        UniqType::First
    };

    writer.write_record(reader.headers()?)?;
    let mut known_entries: BTreeMap<String, csv::StringRecord> = BTreeMap::new();
    for file_row in reader.records() {
        let row = &file_row.wrap_err(format!("Failed to parse {}", &args.input_file))?;
        let maybe_uniq = row.get(unique_col).unwrap();
        if let Some(existing_row) = known_entries.get(maybe_uniq) {
            match &select_type {
                UniqType::First => {}
                UniqType::Max(col) => {
                    let new = row.get(*col).unwrap();
                    let old = existing_row.get(*col).unwrap();
                    if new > old {
                        known_entries.insert(maybe_uniq.to_owned(), row.clone());
                    }
                }
                UniqType::Min(col) => {
                    let new = row.get(*col).unwrap();
                    let old = existing_row.get(*col).unwrap();
                    if new < old {
                        known_entries.insert(maybe_uniq.to_owned(), row.clone());
                    }
                }
            }
        } else {
            known_entries.insert(maybe_uniq.to_owned(), row.clone());
        }
    }
    for record in known_entries.into_values() {
        writer.write_record(&record)?;
    }

    Ok(())
}
