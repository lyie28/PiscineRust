use std::error::Error;
use std::io::Write;
use std::process::Command;
use std::path::Path;
use std::env;

fn echo(args: Vec<&str>) {
    let mut start = 1;
    let size = args.len();

        if size >= 2 && args[1] == "-n" {
            start += 1;
        }
        if size > start {
            let my_echo = &args[start..size];
            for item in my_echo{
                if item == &args[size - 1] {
                    print!("{}", item.replace(&['"', '"', '\''], ""));
                }
                else {
                    print!("{} ", item.replace(&['"', '"', '\''], ""));
                }
            }
            if size == 1 || size > 1 && args[1] != "-n" {
                println!("");
            }
        }
}

fn print_output(ret: std::process::Output) {
    let stdout = String::from_utf8(ret.stdout).unwrap();
            let stderr = String::from_utf8_lossy(&*ret.stderr);
            print!("{}{}", stdout, stderr);
}

fn process_cmd(cmd: &str){
    let args: Vec<&str> = cmd.split(' ').collect();
    let size = args.len();
    
    if size >= 1 && args[0] == "echo" {
        echo(args);
    }
    else if args.len() == 2 && args[0] == "cd" {
        let root = Path::new(args[1]);
        let ret = env::set_current_dir(&root);
        match ret {
            Err(_err) => println!("{}", _err),
            Ok(ret1) => return ret1,
        };
    }
    else if args.len() == 1 && args[0] == "pwd" {
        let path = env::current_dir();
        match path {
            Ok(the_path) => println!("{}", the_path.display()),
            Err(_err) => println!("{}", _err),
        };
    }
    else if args.len() == 1 && args[0] == "env" {
        for (key, value) in env::vars_os() {
            println!("{key:?}: {value:?}");
        }
    }
    else if args.len() == 1 && args[0] == "exit" {
        std::process::exit(0);
    }
    else {
        let mut my_args: Vec<&str> = Vec::new();
        if size > 1 {
            my_args = args[1..size].to_vec();
        }
        let output = Command::new(args[0])
            .args(my_args)
            .output();

        match output {
            Ok(output) => print_output(output),
            Err(_err) => println!("{}",_err),
        };
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let stdin = std::io::stdin();
    let input = &mut String::new();    

    ctrlc::set_handler(move || {
        println!("");
        std::process::exit(130);
    })?;

    loop {
        input.clear();
        print!("->");
        std::io::stdout().flush()?;
        stdin.read_line(input)?;
        let cmd = input.trim();
        process_cmd(&cmd);
    }
}
