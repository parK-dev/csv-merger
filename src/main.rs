extern crate csv;

use std::error::Error;
use std::io;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Collect the paths of all CSV files to be merged
    let mut csv_files: Vec<PathBuf> = Vec::new();
    for arg in &args[1..] {
        let path = PathBuf::from(arg);
        if path.extension().unwrap_or_default() == "csv" {
            csv_files.push(path);
        }
    }

    // Merge the CSV files
    let mut headers_written = false;
    let mut writer = csv::WriterBuilder::new().from_writer(io::stdout());
    for file in csv_files {
        let mut reader = csv::ReaderBuilder::new().from_path(&file)?;
        if !headers_written {
            writer.write_record(reader.headers()?.iter())?;
            headers_written = true;
        }
        for result in reader.records() {
            writer.write_record(result?.iter())?;
        }
    }
    writer.flush()?;

    Ok(())
}
