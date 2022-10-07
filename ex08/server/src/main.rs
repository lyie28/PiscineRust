use std::env;
use std::net::TcpStream;
use std::io::Read;

fn read_file(mut stream: TcpStream, _address: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut buf: &mut [u8] = &mut [0 as u8];
    let mut the_file: Vec<u8> = Vec::new();
    let mut file_name: String = String::new();
    let mut c: u8;
    
    loop {
        let ret = stream.read(buf)?;
        if ret == 0 {
           break;
        }
        c = buf[0];
        if c as char != '3' {
            let tmp: char = buf[0] as char;
            file_name += &tmp.to_string();
        } 
        else {
            break;
        }
    }
    if file_name.len() == 0 {
        return Ok(());
    }
    println!("receiving {}", file_name);
    loop {
        let the_ret = stream.read(&mut buf)?;
        if the_ret == 0 {
            std::fs::write(&file_name, the_file)?;
            println!("{} received and saved.", file_name);
            return Ok(());
        }
        c = buf[0];
        the_file.push(c);
        }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No argument");
        return Ok(());
    }
    let port = &args[1];
    use std::net::TcpListener;
    let listener = TcpListener::bind("127.0.0.1:".to_owned() + &port)?;
    println!("Hosting on port {}, waiting for commands…", &port);
    let (client, addr) = listener.accept()?;
    read_file(client, &addr.to_string())?;
    drop(listener);
    Ok(())
}
