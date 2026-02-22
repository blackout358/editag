use std::{fs, path::PathBuf};

use clap::{command, Arg, ArgAction, ArgMatches};
use regex::Regex;

use crate::models::controls::{AppConfig, ChangeSet, ModifyAction};

pub struct MyParser;

impl MyParser {
    pub fn generate_commandline_args() -> ArgMatches {
        let matches = command!()
            .arg(Arg::new("file_path").action(ArgAction::Set))
            .arg(
                Arg::new("title")
                    .short('t')
                    .long("title")
                    .help("Set the track title")
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("album")
                    .short('a')
                    .long("album")
                    .help("Set the album")
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("artist")
                    .short('A')
                    .long("artist")
                    .help("Set the artist")
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("album-artist")
                    .short('B')
                    .long("album-artist")
                    .help("Set the album artist")
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("year")
                    .short('y')
                    .long("year")
                    .help("Set the year")
                    .value_parser(clap::value_parser!(i32))
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("track_number")
                    .short('n')
                    .long("track-number")
                    .help("Set the track number")
                    .value_parser(clap::value_parser!(u32))
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("genre")
                    .short('g')
                    .long("genre")
                    .help("Set the genre")
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("cover-art-path")
                    .short('c')
                    .long("cover-art")
                    .help("Set the cover art\n")
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("print")
                    .short('p')
                    .long("print-data")
                    .help("View files existing id3 data")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("list-genres")
                    .long("list-genres")
                    .help("List ID3 genres (ignores other flags)")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("delete-tag")
                    .short('d')
                    .long("delete-tag")
                    .help("Delete a frame by passing in its id")
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("delete-all")
                    .short('D')
                    .long("delete-all-tags")
                    .help("Delete all frames")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("recursive")
                    .short('r')
                    .help("Run command on every mp3 file in the specified directory or current directory if none is provided")
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("format-file")
                    .short('f')
                    .long("format-file")
                    .help("Format file to 'TN - TRACK TITLE' TN = track number\n")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("v23")
                    .long("v23")
                    .help("Attempts to save tag as ID3v2.3 instead of ID3v2.4")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("v22")
                    .long("v22")
                    .help("Attempts to save tag as ID3v2.2 instead of ID3v2.4\n")
                    .action(ArgAction::SetTrue),
            )
            .arg(Arg::new("custom-flag").short('C').long("custom").num_args(2).value_names(["Frame id", "Value"]).help("Set a custom frame and its value"))
            .get_matches();
        matches
    }

    pub fn parse_command(matches: &ArgMatches) -> (ChangeSet, AppConfig) {
        let mut actions = vec![];

        if let Some(v) = matches.get_one::<String>("title") {
            actions.push(ModifyAction::Title(v.clone()));
        }
        if let Some(v) = matches.get_one::<String>("artist") {
            actions.push(ModifyAction::Artist(v.clone()));
        }
        if let Some(v) = matches.get_one::<String>("album") {
            actions.push(ModifyAction::Album(v.clone()));
        }
        if let Some(v) = matches.get_one::<String>("album-artist") {
            actions.push(ModifyAction::AlbumArtist(v.clone()));
        }
        if let Some(v) = matches.get_one::<String>("genre") {
            actions.push(ModifyAction::Genre(v.clone()));
        }
        if let Some(v) = matches.get_one::<i32>("year") {
            actions.push(ModifyAction::Year(*v));
        }
        if let Some(v) = matches.get_one::<u32>("track_number") {
            actions.push(ModifyAction::TrackNumber(*v));
        }
        if let Some(v) = matches.get_one::<String>("cover-art-path") {
            actions.push(ModifyAction::CoverArt(PathBuf::from(v)));
        }
        if let Some(v) = matches.get_one::<String>("delete-tag") {
            actions.push(ModifyAction::DeleteTag(v.clone()));
        }
        if let Some(mut values) = matches.get_many::<String>("custom-flag") {
            let frame_id = values.next().unwrap();
            let frame_content = values.next().unwrap();
            actions.push(ModifyAction::Custom(
                frame_id.clone(),
                frame_content.clone(),
            ));
        }

        let changeset = ChangeSet {
            actions,
            delete_all: matches.get_flag("delete-all"),
            format_file: matches.get_flag("format-file"),
            print_details: matches.get_flag("print"),
        };

        let path_str = PathBuf::from(
            matches
                .get_one::<String>("file_path")
                .cloned()
                .unwrap_or_else(|| "./".to_string()),
        );

        let config = AppConfig {
            target_path: path_str,
            recursive: matches.get_flag("recursive"),
            list_genres: matches.get_flag("list-genres"),
            version: if matches.get_flag("v22") {
                id3::Version::Id3v22
            } else if matches.get_flag("v23") {
                id3::Version::Id3v23
            } else {
                id3::Version::Id3v24
            },
        };

        return (changeset, config);
    }
}
