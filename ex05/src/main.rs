use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    collections::{HashMap},
};

fn build_phonebook(path: &str) -> Result<HashMap<String, String>, Box<dyn Error>>
{
    let mut contacts: HashMap<String, String> = HashMap::new();
    let file = File::open(path)?;
    let reader = BufReader::new(&file);
    for line in reader.lines() {
        
        //error handling
        let my_line: String;
        match line {
            Ok(line) => my_line = line,
            Err(err) => return Err(err.into()),
        };
        
        let values: Vec<&str> = my_line.split(';').collect();
        if values.len() == 2 {
            contacts.insert(values[0].to_string(), values[1].to_string());
        }
        else {
            println!("Detected empty line or formatting error. Skipping entry...");
        }
    }
    return Ok(contacts);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("!Usage [cargo run -- input_file]");
        return Ok(());
    }
    
    let tmp = build_phonebook(&args[1]);
    
    //error handling
    let contacts;
    match tmp {
        Ok(tmp) => contacts = tmp,
        Err(err) => return Err(err.into()),
    };

    if contacts.len() == 0 {
        println!("Your phone book is empty");
        return Ok(());
    }

    println!("Your phone book contains:");
    for (contact, number) in contacts.iter() {
        println!("Contact: <{}>, Number: <{}>", contact, number);
    }

    loop {
        println!("Please enter a name");
        let mut name = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut name)?;
        name = name.trim().to_string();
        match contacts.get(&name) {
            Some(number) => println!("{}", number),
            None => println!("No result"),
        }
    }
}
