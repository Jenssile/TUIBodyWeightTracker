use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::OpenOptions, io::Write, path::Path};

const FILE_PATH: &str = "weights.csv";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub date: NaiveDate,
    pub weight: f32,
}

pub fn read_entries() -> Result<Vec<Entry>, Box<dyn Error>> {
    if !Path::new(FILE_PATH).exists() {
        return Ok(vec![]);
    }
    let mut rdr = csv::Reader::from_path(FILE_PATH)?;
    let mut entries = vec![];
    for result in rdr.deserialize() {
        let entry: Entry = result?;
        entries.push(entry);
    }
    Ok(entries)
}

pub fn write_entry(weight: f32) -> Result<(), Box<dyn Error>> {
    let today = chrono::Local::now().date_naive();

    let mut entries = read_entries()?;
    if entries.iter().any(|e| e.date == today) {
        println!("Entry for today already exists.");
        return Ok(());
    }

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(!Path::new(FILE_PATH).exists())
        .from_writer(OpenOptions::new().create(true).append(true).open(FILE_PATH)?);

    wtr.serialize(Entry {
        date: today,
        weight,
    })?;
    wtr.flush()?;
    Ok(())
}
