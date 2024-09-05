use std::fs;
use colored::Colorize;
use clap::{Parser};

#[derive(Parser)]
pub struct Args {
    // #[clap(short = 'p', long, default_value = "./")]
    #[clap(default_value = "./")]
    pub file_path: String,

    #[clap(short = 'l', long, help = "Long listing format")]
    pub long_format: bool,
}

pub struct ListItem{
    pub file_name: String,
    pub file_type: String,
    pub color: Option<String>,
    pub unicode_icon: String,
}
impl ListItem{
    pub fn build(file_name: String, file_type: &str, color: Option<String>, unicode_icon: String) -> ListItem{
        ListItem{
            file_name: file_name.to_string(),
            file_type: file_type.to_string(),
            color: color,
            unicode_icon: unicode_icon.to_string(),
        }
    }
}


pub fn list_files(args: Args) -> std::io::Result<()>{
    let paths = fs::read_dir(&args.file_path)?;
    let mut list_items: Vec<ListItem> = Vec::new();

    for item in paths {
        match item {
            Ok(entry) => {
                let path = entry.path();
                if let Some(file_name) = path.file_name(){ 
                    let file_type = if path.is_dir() {"Directory"} else {"File"};
                    let color = if path.is_dir() {"blue".to_string()} else {"white".to_string()};
                    let unicode_icon =  if path.is_dir() {"\u{1f4c1}".to_string()} else {"\u{1f4c4}".to_string()}; 

                        list_items.push(ListItem::build(
                        file_name.to_string_lossy().to_string(),
                        file_type,
                        Some(color),
                        unicode_icon,
                    ))
                }
            }
            Err(e) => {
                eprintln!("Error reading entry: {}", e)
            }
        }
    }
    if args.long_format{
        println!("looooooong");
    }
    // file_strings.sort();
    list_items.sort_by_key(|item| item.file_type.clone());
    for file in list_items{
        if file.file_type == "Directory"{
            println!("{}  {}", file.unicode_icon.bright_white(), file.file_name.blue());
        }else{
            println!("{}  {}", file.unicode_icon.bright_white(), file.file_name.bright_white());
        }
    };
    Ok(())
}
