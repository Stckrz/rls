use std::fs;
use colored::Colorize;
pub struct Config{
    pub file_path: String
}
impl Config{
    pub fn build(args: &[String]) -> Result<Config, &'static str>{
        let default_file_path = "./".to_string();

        let file_path = if args.len() > 1 {
            args[1].clone()
        }else{
            default_file_path
        };

        Ok(Config {file_path})
    }
}
pub struct ListItem{
    pub file_name: String,
    pub file_type: String,
    pub color: Option<String>,
    pub unicode_icon: String
}
impl ListItem{
    pub fn build(file_name: String, file_type: &str, color: Option<String>, unicode_icon: String) -> ListItem{
        ListItem{
            file_name: file_name.to_string(),
            file_type: file_type.to_string(),
            color: color,
            unicode_icon: unicode_icon.to_string()
        }
    }
}

pub fn list_files(config: Config) -> std::io::Result<()>{
    let paths = fs::read_dir(&config.file_path)?;
    let mut list_items: Vec<ListItem> = Vec::new();

    for item in paths {
        match item {
            Ok(item) => {
                if let Some(file_name) = item.path().file_name(){ 
                // if let Some(file_name) = item.path().extension(){
                    if item.path().is_dir(){
                        list_items.push(
                            ListItem::build(
                                file_name.to_string_lossy().to_string(),
                                "Directory",
                                Some("blue".to_string()),
                                "\u{1f4c1}".to_string()
                                
                            )
                        )
                    } else{
                        list_items.push(
                            ListItem::build(
                                file_name.to_string_lossy().to_string(),
                                "File",
                                Some("red".to_string()),
                                "\u{1f4c4}".to_string(),
                            )
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading entry: {}", e)
            }
        }
    }
    // file_strings.sort();
    list_items.sort_by_key(|item| item.file_type.clone());
    for file in list_items{
        if file.file_type == "Directory"{
        println!("{}  {}", file.unicode_icon, file.file_name.blue())
        }else{
        println!("{}  {}", file.unicode_icon, file.file_name.white())
        }
    };
    Ok(())
}

