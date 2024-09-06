use stckrls::list_files;
use std::process;
use stckrls::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    if let Err(err) = list_files(args){
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    }
}
