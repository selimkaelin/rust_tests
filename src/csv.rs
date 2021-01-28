extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

// derive Debug, what for?
#[derive(Debug, Deserialize)]


pub fn run() -> Result<(), Box<Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(file_path)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

pub fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("Expected 1 argument, but got none.")),
        Some(file_path) => Ok(file_path),
    }
}

// fn read_csv() -> Result<Vec<T>, Box<Error>> {
//     let file_path = get_first_arg()?;
//     let mut data = vec![vec![0.0; 8]; 1];
//     let mut reader = csv::ReaderBuilder::new()
//         .delimiter(b';')
//         .from_path(file_path)?;
//     for result in reader.records() {
//         let record= result?;
//         data.push(record);
//     }
//
//     Ok(data)
// }

// fn run() -> Result<(), Box<Error>> {
//     let mut rdr = csv::Reader::from_reader(io::stdin());
//     for result in rdr.records() {
//         let record = result?;
//         println!("{:?}", record);
//         // match result {
//         //     Err(err) => return Err(From::from(err)),
//         //     Ok(record) => {
//         //         println!("{:?}", record);
//         //     }
//         // }
//     }
//     Ok(())
// }