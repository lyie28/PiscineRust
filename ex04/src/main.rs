use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::error::Error;
use std::fs::File;

pub trait Copy: Clone { }
enum MyEnum {
    Float(f32),
    Bit(u8),
    Str(String),
}

fn get_format_table(str: &str, out: &mut Vec<char>) -> Result<i32, Box<dyn Error>> {

    if str.len() == 0 {
        println!("Your file had no columns in header row");
        return Ok(-1);
    }

    let values: Vec<&str> = str.split(';').collect();
    if values.len() == 0 {
        println!("Your file had no columns in header row");
        return Ok(-1);
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
        else {
            return Ok(-1)
        }
    }
    return Ok(0);

}

/*display implementation so can print out vectors*/
impl std::fmt::Display for MyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyEnum::Str(v) => v.fmt(f),
            MyEnum::Float(v) => v.fmt(f),
            MyEnum::Bit(v) => v.fmt(f),
        }
    }
}

/*clone implementation so can push inner vec to outer without error*/
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

    //check args
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage [executable path] [input file]!");
        return Ok(());
    }

    //open and read file
    let path = &args[1];
    let file = File::open(path)?;
    let reader = BufReader::new(&file);

    let mut outer_tab: Vec<Vec<MyEnum>> = Vec::new();
    let mut tab = vec![];    
    let mut count = 0;
    
    for line in reader.lines() {
        //error handling
        let my_line: String;
        match line {
            Ok(line) => my_line = line,
            Err(err) => return Err(err.into()),
        };
        
        //create a formatting table which stores order of formats
        if count == 0 {
            let ret = get_format_table(&my_line, &mut tab);
            match ret {
                Ok(ret) => ret,
                Err(err) => return Err(err.into()),
            };
            if ret.unwrap() == -1 {
                println!("Unexpected value in header column");
                return Ok(());
            }
        }
        
        //loop over cells
        let values: Vec<&str> = my_line.split(';').collect();
        outer_tab.push(Vec::new());
        for item in values {
            if tab[count] == 's' || count == 0 {
                outer_tab[count].push(MyEnum::Str(item.to_string()));
            }
            else if tab[count] == 'u' && count != 0 {
                outer_tab[count].push(MyEnum::Bit(item.as_bytes()[0]));
            }
            else if tab[count] == 'f' && count != 0{
                outer_tab[count].push(MyEnum::Float(item.parse::<f32>().unwrap()));
            }
        }
        count += 1;
        println!("");
    }

    if count == 0 {
        println!("Your file was empty!");
        return Ok(());
    }

    //print out vector of vectors
    println!("My vector or vectors is: ");
    for rows in outer_tab {
        let cells : &Vec<MyEnum> = &rows;
            for cell in cells {
                print!("{};", cell);
            }
        println!("");
    }
    return Ok(());
}