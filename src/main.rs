mod syncer;

use std::{
    fs::{self, File},
    path::PathBuf,
};

use syncer::Syncer;

use serde::{Deserialize, Serialize};
use toml_edit::Document;

#[derive(Debug, Serialize, Deserialize)]
struct Preset {
    name: String,
    source: String,
    dest: String,
}

impl Preset {
    fn new(name: String, source: String, dest: String) -> Self {
        Self { name, source, dest }
    }

    fn register(&self) {
        let config_file_path = get_config_file_path();
        let toml = fs::read_to_string(&config_file_path).unwrap();
        let mut doc = toml.parse::<Document>().unwrap();

        doc["presets"][&self.name]["source"] = toml_edit::value(&self.source);
        doc["presets"][&self.name]["dest"] = toml_edit::value(&self.dest);

        fs::write(&config_file_path, doc.to_string()).unwrap();
    }
}

fn get_config_file_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".local/share/san/config.toml");
    path
}

fn create_config_file() {
    let file_path = get_config_file_path();
    let file_contents = match fs::read_to_string(&file_path) {
        Ok(contents) => contents,
        Err(_) => {
            let dir_path = file_path.parent().unwrap();
            fs::create_dir_all(&dir_path).unwrap();
            File::create(&file_path).unwrap();
            "".to_string()
        }
    };
    if !file_contents.contains("[presets]") {
        fs::write(
            &file_path,
            r#"
            [presets]
            "#,
        )
        .unwrap();
    }
}

fn main() {
    create_config_file();
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        let message = r#"
        Usage: san <command> [args]

        Commands:
            add <name> <source> <dest> - Add a new preset
            sync <name> - Sync a preset
            list - List all presets
            help - Show this message

        Examples:
            san add "My Preset" "/path/to/folder" "remote:/path/to/folder"
            san sync "My Preset"
        "#;

        println!("{}", message);
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 5 {
                println!("Not enough arguments");
                return;
            }

            let name = &args[2];
            let source = &args[3];
            let dest = &args[4];

            let preset = Preset::new(name.to_string(), source.to_string(), dest.to_string());
            preset.register();
        }
        "sync" => {
            if args.len() < 3 {
                println!("Not enough arguments");
                return;
            }

            let name = &args[2];

            let file_path = get_config_file_path();
            let toml = fs::read_to_string(&file_path).unwrap();
            let doc = toml.parse::<Document>().unwrap();

            let source = doc["presets"][name]["source"].as_str().unwrap();
            let dest = doc["presets"][name]["dest"].as_str().unwrap();

            let syncer = Syncer::new(source, dest);
            syncer.watch();
        }
        "list" => {
            let file_path = get_config_file_path();
            let toml = fs::read_to_string(&file_path).unwrap();
            let doc = toml.parse::<Document>().unwrap();

            let presets = doc["presets"].as_table().unwrap();

            for (name, _) in presets {
                println!("{}", name);
            }
        }
        "help" => {
            let message = r#"
            Usage: san <command> [args]

            Commands:
                add <name> <source> <dest> - Add a new preset
                sync <name> - Sync a preset
                list - List all presets
                help - Show this message

            Examples:
                san add "My Preset" "/path/to/folder" "remote:/path/to/folder"
                san sync "My Preset"
            "#;

            println!("{}", message);
            return;
        }
        _ => {
            println!("Invalid command");
            return;
        }
    }
}
