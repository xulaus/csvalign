use clap::Parser;
use std::{collections::BTreeMap, error::Error};

#[derive(Debug)]
enum UniqType {
    First,
    Max(usize),
    Min(usize),
}

/// Take a single csv file and make one of the columns unique
#[derive(Parser, Debug)]
pub struct MakeUniqArgs {
    input_file: String,
    #[arg(short, long)]
    verbose: bool,
    /// Column to ensure every entry is unique in
    unique_column: String,

    /// Choose which enty to keep by keeping the maximum value in this column
    #[arg(long, group = "select")]
    max_by: Option<String>,

    /// Choose which enty to keep by keeping the minimum value in this column
    #[arg(long, group = "select")]
    min_by: Option<String>,

    #[arg(short, long)]
    out: String,
}

pub fn csvmakeuniq(args: MakeUniqArgs) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(args.input_file)?;
    let mut writer = csv::Writer::from_path(args.out)?;
    writer.write_record(reader.headers()?)?;
    let this_headers = reader.headers()?;
    let unique_col = this_headers
        .iter()
        .position(|x| x == args.unique_column)
        .ok_or("Couldn't find column in input csv")?;

    let select_type = if let Some(col) = args.max_by {
        let pos = this_headers
            .iter()
            .position(|x| x.trim() == col)
            .ok_or("Couldn't find max-by column in input csv")?;
        UniqType::Max(pos)
    } else if let Some(col) = args.min_by {
        let pos = this_headers
            .iter()
            .position(|x| x.trim() == col)
            .ok_or("Couldn't find min-by column in input csv")?;
        UniqType::Min(pos)
    } else {
        UniqType::First
    };
    let mut known_entries: BTreeMap<String, csv::StringRecord> = BTreeMap::new();
    for file_row in reader.records() {
        let row = &file_row?;
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
