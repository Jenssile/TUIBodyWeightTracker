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

    let entries = read_entries()?;
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_FILE: &str = "test_weights.csv";

    fn setup() {
        // Remove test file if exists before each test
        let _ = fs::remove_file(TEST_FILE);
    }

    fn teardown() {
        // Cleanup test file after tests
        let _ = fs::remove_file(TEST_FILE);
    }

    fn write_entry_test(weight: f32, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let today = chrono::Local::now().date_naive();
        let mut entries = read_entries_test(file_path)?;
        if entries.iter().any(|e| e.date == today) {
            return Ok(());
        }

        let mut wtr = csv::WriterBuilder::new()
            .has_headers(!std::path::Path::new(file_path).exists())
            .from_writer(std::fs::OpenOptions::new().create(true).append(true).open(file_path)?);

        wtr.serialize(Entry {
            date: today,
            weight,
        })?;
        wtr.flush()?;
        Ok(())
    }

    fn read_entries_test(file_path: &str) -> Result<Vec<Entry>, Box<dyn std::error::Error>> {
        if !std::path::Path::new(file_path).exists() {
            return Ok(vec![]);
        }
        let mut rdr = csv::Reader::from_path(file_path)?;
        let mut entries = vec![];
        for result in rdr.deserialize() {
            let entry: Entry = result?;
            entries.push(entry);
        }
        Ok(entries)
    }

    #[test]
    fn test_write_and_read_entry() {
        setup();
        // Write entry
        write_entry_test(75.5, TEST_FILE).expect("Failed to write entry");
        // Read entries
        let entries = read_entries_test(TEST_FILE).expect("Failed to read entries");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].weight, 75.5);
        teardown();
    }

    #[test]
    fn test_duplicate_date_prevention() {
        setup();
        write_entry_test(70.0, TEST_FILE).unwrap();
        // Try to add another entry for the same day
        write_entry_test(72.0, TEST_FILE).unwrap();
        let entries = read_entries_test(TEST_FILE).unwrap();
        // Should only have one entry
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].weight, 70.0);
        teardown();
    }
}
