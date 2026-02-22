mod models;
use std::fs;

use models::parser::MyParser;

use crate::models::{controls::ID3_GENRES, track::Track};

fn print_genres() {
    println!("{0: <25} | {1: <25}", "ID: Genre", "ID: Genre");
    println!("{:-<26}|{:-<26}", "", "");

    let offset = (ID3_GENRES.len() + 1) / 2;
    for i in 0..offset {
        let left = format!("{}", ID3_GENRES[i]);
        let right = if i + offset < ID3_GENRES.len() {
            format!("{}: {}", i + offset, ID3_GENRES[i + offset])
        } else {
            "".to_string()
        };
        println!("{0: <25} | {1: <25}", left, right);
    }
}

fn main() {
    let parser = MyParser::generate_commandline_args();
    let (change_set, config) = MyParser::parse_command(&parser);

    if config.list_genres {
        print_genres();
        return;
    }

    let mut files = Vec::new();
    if config.recursive && config.target_path.is_dir() {
        if let Ok(entries) = fs::read_dir(&config.target_path) {
            for entry in entries.flatten() {
                if entry.path().extension().map_or(false, |e| e == "mp3") {
                    files.push(entry.path());
                }
            }
        }
    } else if !config.recursive && config.target_path.is_dir() {
        eprintln!("Invalid file, use 'editag --help' for help");
    } else {
        files.push(config.target_path.clone());
    }

    for file_path in files {
        match Track::load(file_path) {
            Ok(mut track) => match track.apply(&change_set, config.version) {
                Ok(modified) => {
                    if modified {
                        if let Err(e) = track.save(config.version) {
                            eprintln!("Failed the save {:?}: {}", track.path, e)
                        }
                    }
                    if change_set.format_file {
                        if let Err(e) = track.format_filename() {
                            eprintln!("Failed to format filename {:?}: {}", track.path, e);
                        };
                    }
                    println!("Processed: {:?}\n", track.path)
                }
                Err(e) => {
                    eprintln!("{}", e)
                }
            },
            Err(e) => eprintln!("Error loading file: {}", e),
        }
    }
}
