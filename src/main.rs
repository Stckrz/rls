use rls::list_files;
use std::env;
use std::process;
use rls::Config;
// use clap::{Parser};
use clap::Parser;

#[derive(Parser)]
struct Args {
    file_path: String
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // let args = Args::parse();
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = list_files(config){
        eprintln!("error occurred {}", e)
    }
}
