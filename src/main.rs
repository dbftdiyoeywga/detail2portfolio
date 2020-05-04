use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::io;
use std::process;

use csv::StringRecord;

fn parse() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(io::stdin());

    let mut wtr = csv::Writer::from_path(file_path)?;

    let mut category: String = "None".to_string();
    for result in rdr.records() {
        let record = result?;
        if record.len() == 2 {
            category = record[0].to_string();
        } else if (record.len() == 11) & (&record[1] != "買付日") {
            let mut acc = StringRecord::new();
            acc.push_field(&category);
            for field in record.iter() {
                acc.push_field(field);
            }
            wtr.write_record(acc.iter())?;
        }
    }
    wtr.flush()?;
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = parse() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
