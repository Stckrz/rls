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

    // let args: Vec<String> = env::args().collect();
    // let config = Config::build(&args).unwrap_or_else(|err|{
    // let config = Config::build(args).unwrap_or_else(|err|{
    //     println!("problem parsing arguments: {}", err);
    //     process::exit(1);
    // });
    //
    // if let Err(e) = list_files(config){
    //     eprintln!("error occurred {}", e)
    // }
}
