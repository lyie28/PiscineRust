use std::{env};
use std::error::Error;
use std::fs::File;
use std::any::{Any, TypeId};
use csv::ByteRecord;

fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage [executable path] [input file]!");
        return Ok(());
    }

    let path = &args[1];
    let my_vec: Vec<String>;
    let file = File::open(path)?;

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(file);

        let mut result: ByteRecord;  
        for result in rdr.read_byte_record()? {
        let ret = ByteRecord::from(rdr.records());
        let headers = rdr.headers()?;
            for header in *headers {
            /*if header.type_id() == TypeId::of::<u32>() {
                my_vec.push(header.toString()?);
            } else if header.type_id() == TypeId::of::<String>() {
                my_vec.push(header);
            }*/
                println!("{}", header);
            }
    }

    /* match */
    return Ok(());
}