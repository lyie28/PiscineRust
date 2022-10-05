use std::io::BufReader;
use std::io::BufRead;
use std::{env};
use std::error::Error;
use std::fs::File;

pub trait Copy: Clone { }
enum MyEnum {
    Float(f32),
    Bit(u8),
    Str(String),
}

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

impl std::fmt::Display for MyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyEnum::Str(v) => v.fmt(f),
            MyEnum::Float(v) => v.fmt(f),
            MyEnum::Bit(v) => v.fmt(f),
        }
    }
}

impl Clone for MyEnum {
    fn clone(&self) -> MyEnum {
        match self {
            MyEnum::Str(a) => MyEnum::Str(a.clone()),
            MyEnum::Bit(a) => MyEnum::Bit(a.clone()),
            MyEnum::Float(a) => MyEnum::Float(a.clone()),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage [executable path] [input file]!");
        return Ok(());
    }
    let mut outer_vec: Vec<Vec<MyEnum>> = Vec::new();
    let first = 0;
    let path = &args[1];
    let file = File::open(path)?;
    let reader = BufReader::new(&file);
    let mut tab = vec![];
    let mut count = 0;
    let mut inner_tab: [Vec<MyEnum>; 10 ]= Default::default();
    //let ret = count_new_lines(&reader);
    for line in reader.lines() {
        let my_line = line.unwrap();
        if first == 0 {
            let _ret = get_format_table(&my_line, &mut tab);
        }
        let values: Vec<&str> = my_line.split(';').collect();
        for item in values {
            inner_tab[count] = Vec::new();
            if tab[count] == 's' || count == 0 {
                inner_tab[count].push(MyEnum::Str(item.to_string()));
            }
            else if tab[count] == 'u' && count != 0 {
                inner_tab[count].push(MyEnum::Bit(item.as_bytes()[0]));
            }
            else if tab[count] == 'f' && count != 0{
                inner_tab[count].push(MyEnum::Float(item.parse::<f32>().unwrap()));
            }
            //let printable : &Vec<MyEnum> = &inner_tab[count];
            //for item in printable {
            //    print!("{};", item);
            //}
        }
        outer_vec.push(inner_tab[count].clone());
        count += 1;
        println!("");
    }

    return Ok(());
}