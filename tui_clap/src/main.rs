use clap::Parser;
use std::{fs::OpenOptions, io::Read, io::Write};


/// Simple program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)]
   name: String,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   passcode: i32,
}

fn main() {
    let args = Args::parse();
    
    let mut file_content = String::new();
    let mut auth = OpenOptions::new()
    .read(true)
    .append(true)
    .create(true)
    .open("secure.txt").unwrap();

    let mut data = OpenOptions::new().read(true)
    .append(true)
    .create(true)
    .open("data.txt").unwrap();
    
    writeln!(auth, "{} {}", args.name, args.passcode);

    data.read_to_string(&mut file_content);

    println!("{}", file_content);

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name)
    // }
}
