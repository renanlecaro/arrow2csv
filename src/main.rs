use arrow::ipc::reader::FileReader;
use arrow::csv::Writer;
use std::fs::File;
use std::io::{BufReader};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    
    // Open the Arrow file
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    
    // Read the Arrow file using FileReader
    let arrow_reader = FileReader::try_new(&mut reader, None)?;

    // Create a CSV writer with the same filename but with .csv extension
    let csv_filename = format!("{}.csv", filename);
    let file = File::create(csv_filename)?;
    let mut csv_writer = Writer::new(file);

    // Process each record batch in the Arrow file
    for batch in arrow_reader { 
        csv_writer.write(&batch?).unwrap();
    }

    Ok(())
}
