mod models;
use std::path::{Path, PathBuf};

use clap::{command, Arg, ArgAction, ArgMatches};
use models::track::Track;
use serde_json::Value;

fn build_track(matches: &ArgMatches) -> Track {
    let file_path: Option<PathBuf> = match matches.get_one::<String>("file_path") {
        Some(name) => Some([r"./", name].iter().collect()),
        None => None,
    };

    let image_path: Option<PathBuf> = match matches.get_one::<String>("album_art") {
        Some(name) => Some([r"./", name].iter().collect()),
        None => None,
    };
    let ts = Track::new(
        file_path,
        matches.get_one::<String>("title").cloned(),
        matches.get_one::<String>("album").cloned(),
        matches.get_one::<String>("artist").cloned(),
        matches.get_one::<i32>("year").cloned(),
        matches.get_one::<i32>("track_number").cloned(),
        matches.get_one::<String>("genre").cloned(),
        image_path,
    );
    ts
}

fn apply_tags(music_track: &Track) -> () {
    let json = serde_json::to_string(music_track).unwrap();
    let val: Value = serde_json::from_str(&json).unwrap();

    if let Value::Object(map) = val {
        for (key, val) in map {
            match val {
                Value::Null => println!("Field name: {}, Value: Null", key),
                _ => println!("Field Name: {}, Value: {}", key, val),
            }
        }
    }
}

fn main() {
    let matches = command!()
        .arg(Arg::new("file_path").action(ArgAction::Set))
        .arg(
            Arg::new("title")
                .short('t')
                .long("title")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("album")
                .short('a')
                .long("album")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("artist")
                .short('A')
                .long("artist")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("year")
                .short('y')
                .long("year")
                .value_parser(clap::value_parser!(i32))
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("track_number")
                .short('n')
                .long("track-number")
                .value_parser(clap::value_parser!(i32))
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("genre")
                .short('g')
                .long("genre")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("album_art")
                // .short('A')
                .long("add-image")
                .action(ArgAction::Set),
        )
        .get_matches();

    let music_track = build_track(&matches);
    apply_tags(&music_track);

    music_track.update_track();
}
