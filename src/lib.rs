use std::fs;
use colored::Colorize;
use clap::Parser;
use std::os::unix::fs::PermissionsExt;
use chrono::{DateTime, Local};
use comfy_table::*;
use comfy_table::presets::NOTHING;

#[derive(Parser)]
pub struct Args {
    #[clap(default_value = "./")]
    pub file_path: String,

    #[clap(short = 'l', long, help = "Long listing format")]
    pub long_format: bool,
}

pub struct ListItem{
    pub file_name: String,
    pub file_type: String,
    pub unicode_icon: String,
    pub mod_date: String,
    pub file_size: String,
    pub permissions: String,
    pub metadata: fs::Metadata,
}
impl ListItem{
    pub fn build(file_name: String, file_type: &str, unicode_icon: String, metadata: fs::Metadata) -> ListItem{
            let date: String = get_modified_date(&metadata);
            let size: String = metadata.len().to_string();
            let permissions: String = readable_permissions(metadata.permissions().mode());
        ListItem{
            file_name: file_name.to_string(),
            file_type: file_type.to_string(),
            unicode_icon: unicode_icon.to_string(),
            mod_date: date,
            file_size: size,
            permissions: permissions,
            metadata: metadata,
        }
    }
}
fn get_modified_date(metadata: &fs::Metadata) -> String{
    match metadata.modified(){
        Ok(system_time) => {
            let datetime: DateTime<Local> = DateTime::from(system_time);
            datetime.format("%m-%d %H:%M").to_string()
        },
            Err(_) => String::from("Error getting modification time")
    }
}


fn readable_permissions(mode: u32) -> String{
    //I did not write this block, and its a bit hard for me to understand why rust gives you the
    //option to print rwx format in debug mode, but you have to do bit conversion to actually print
    //this in a human-readable manner..
    let mut permissions = String::new();
    permissions.push(if mode & 0o400 != 0 {'r'} else {'-'});
    permissions.push(if mode & 0o200 != 0 {'w'} else {'-'});
    permissions.push(if mode & 0o100 != 0 {'x'} else {'-'});

    permissions.push(if mode & 0o040 != 0 {'r'} else {'-'});
    permissions.push(if mode & 0o020 != 0 {'w'} else {'-'});
    permissions.push(if mode & 0o010 != 0 {'x'} else {'-'});

    permissions.push(if mode & 0o004 != 0 {'r'} else {'-'});
    permissions.push(if mode & 0o002 != 0 {'w'} else {'-'});
    permissions.push(if mode & 0o001 != 0 {'x'} else {'-'});
    permissions
}
fn file_name_colorize(file_type: &str, unicode_icon: &str, file_name: &str ) -> String{

    if file_type == "Directory"{
        format!("{}  {}", unicode_icon.bright_white(), file_name.blue())
    }else{
        format!("{}  {}", unicode_icon.bright_white(), file_name.bright_white())
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
                    let unicode_icon =  if path.is_dir() {"\u{1f4c1}"} else {"\u{1f4c4}"}; 
                    let metadata = path.metadata()?;

                        list_items.push(ListItem::build(
                        file_name.to_string_lossy().to_string(),
                        file_type,
                        unicode_icon.to_string(),
                        metadata,
                    ))
                }
            }
            Err(e) => {
                eprintln!("Error reading entry: {}", e)
            }
        }
    }
    list_items.sort_by_key(|item| item.file_type.clone());
    if args.long_format{
        let mut table = Table::new();
        table.load_preset(NOTHING);
        

        for file in list_items{
            let file_name: String =file_name_colorize(&file.file_type, &file.unicode_icon, &file.file_name); 
                table.add_row(vec![

                Cell::new(file_name),
                Cell::new(file.permissions),
                Cell::new(file.file_size),
                Cell::new(file.mod_date),
                ]);
        println!("{:?}", file.metadata.permissions());
            }
        println!("{}", table);
    }else{
        for file in list_items{
            println!("{}", file_name_colorize(&file.file_type, &file.unicode_icon, &file.file_name))
        };
    }
    Ok(())
}
