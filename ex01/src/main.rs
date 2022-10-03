use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No argument");
        return;
    }

    let mut count = 0;
    for str in args
    {
        if count != 0 {
            print!(", ");
        }
        else{
            count = 1;
        }
        print!("{str}");
    }
    println!("");
    return;
}