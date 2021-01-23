// Build a CSV reader
use std::error::Error;
use csv::ReaderBuilder;

pub fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_path("hausstock_history.csv")?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}