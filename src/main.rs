use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::{env, process};

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let get_file = || -> Result<Box<dyn BufRead>, Box<dyn Error>> {
        match env::args_os().nth(1) {
            Some(filename) => {
                if filename == "-" {
                    Ok(Box::new(BufReader::new(io::stdin())))
                } else {
                    let f = File::open(filename)?;
                    Ok(Box::new(BufReader::new(f)))
                }
            }
            None => Err(From::from("CSV file or '-' (for stdin) is required")),
        }
    };

    let mut reader = csv::Reader::from_reader(get_file()?);
    for record in reader.records() {
        println!("{:?}", record?);
    }

    Ok(())
}
