mod syncer;

use std::fs;

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
        let toml = fs::read_to_string("config.toml").unwrap();
        let mut doc = toml.parse::<Document>().unwrap();

        doc["presets"][&self.name]["source"] = toml_edit::value(&self.source);
        doc["presets"][&self.name]["dest"] = toml_edit::value(&self.dest);

        fs::write("config.toml", doc.to_string()).unwrap();
    }
}
fn main() {
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
            let toml = fs::read_to_string("config.toml").unwrap();
            let doc = toml.parse::<Document>().unwrap();

            let source = doc["presets"][name]["source"].as_str().unwrap();
            let dest = doc["presets"][name]["dest"].as_str().unwrap();

            let syncer = Syncer::new(source, dest);
            syncer.watch();
        }
        "list" => {
            let toml = fs::read_to_string("config.toml").unwrap();
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
