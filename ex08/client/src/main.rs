use std::net::TcpStream;
use std::env;
use std::io::Write;
use std::fs;
use std::path::Path;


fn send_file(mut stream: TcpStream, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    //check file exists
    if Path::new(file_name).exists() == false {
        println!("File does not exist");
        return Ok(());
    }
    
    let break_char: u8 = 3;
    println!("Client: Uploading {} to the server…", file_name);
    write!(stream, "{}", file_name)?;
    write!(stream, "{}", break_char)?;
    let input_file = fs::read(file_name)?;
    stream.write_all(&input_file)?;
    println!("Client: Uploaded…");
   Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let args: Vec<String> = env::args().collect();
    if args.len() <= 3 {
        println!("Usage: cargo run -- [server address] [port]");
        return Ok(());
    }
    
    let file_name = &args[3];
    let address = args[1].to_string() + ":" + &args[2].to_string();
    match TcpStream::connect(&address) {
        Ok(server) => {
            send_file(server, &file_name)?;
        }
        Err(error) => {
            println!("Connection failed : {}", error);
        }
    }
    Ok(())
}
