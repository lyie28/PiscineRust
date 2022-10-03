use std::io::BufReader;
use std::io::BufRead;
use std::{env};
use std::error::Error;
use std::fs::File;


fn get_format_table(str: &str, out: &mut Vec<char>) -> Result<u32, Box<dyn Error>> {

    let values: Vec<&str> = str.split(';').collect();
    if values.len() == 0 {
        println!("Your file had no columns in header row");
        return Ok(1);
    }
    for item in values {
        if item == "String" {
            out.push('s');
        }
        else if item == "u8" {
            out.push('u');
        }
        else if item == "f32" {
            out.push('f');
        }
    }
    return Ok(0);

}
fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage [executable path] [input file]!");
        return Ok(());
    }

    let first = 0;
    let path = &args[1];
    let file = File::open(path)?;
    let reader = BufReader::new(&file);
    let mut tab = vec![];
    for line in reader.lines() {
        let my_line = line.unwrap();
        if first == 0 {
            let _ret = get_format_table(&my_line, &mut tab);
        }
        for items in tab.iter() {
            println!("Check: {}", items);
        }
        //for items in row
        //get type from index
        //match
        //if this
        //if this
        //if this
        //add to vector
    }

    return Ok(());
}