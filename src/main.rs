use arrow::ipc::reader::FileReader; 
use arrow::csv::Writer;
use std::fs::File;
use std::io::{BufReader};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the Arrow file
    let filename = "input.arrow";
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    
    // Read the Arrow file using FileReader
    let arrow_reader = FileReader::try_new(&mut reader, None)?;

    // Create a CSV writer
    let file = File::create(format!("{}.csv", filename))?; 
    let mut csv_writer = Writer::new(file); 
  
    // Process each record batch in the Arrow file
    for batch in arrow_reader { 
        csv_writer.write(&batch?).unwrap();
    }

    Ok(())
}
 