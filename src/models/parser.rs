use std::path::PathBuf;

use clap::{command, Arg, ArgAction, ArgMatches};

pub struct MyParser {}

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
                    .value_parser(clap::value_parser!(i32))
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
                Arg::new("path_to_image")
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
                Arg::new("tag to delete")
                    .short('d')
                    .long("delete-tag")
                    .help("Delete a frame by passing in its id\n")
                    .action(ArgAction::Set),
            )
            .get_matches();
        matches
    }

    pub fn parse_command(matches: &ArgMatches) -> super::track::Track {
        let mut tag: Option<id3::Tag> = None;
        let file_path: Option<PathBuf> = match matches.get_one::<String>("file_path") {
            Some(name) => {
                let path: PathBuf = [r"./", name].iter().collect();
                tag = Some(id3::Tag::read_from_path(&path).expect("Error reading mp3 tag"));

                Some(path)
            }
            None => None,
        };
        let image_path: Option<PathBuf> = match matches.get_one::<String>("path_to_image") {
            Some(name) => Some([r"./", name].iter().collect()),
            None => None,
        };

        let mut ts = super::track::Track::new(
            file_path.clone(),
            tag,
            matches.get_one::<String>("title").cloned(),
            matches.get_one::<String>("album").cloned(),
            matches.get_one::<String>("artist").cloned(),
            matches.get_one::<i32>("year").cloned(),
            matches.get_one::<i32>("track_number").cloned(),
            matches.get_one::<String>("genre").cloned(),
            image_path,
        );

        if let Some(value) = matches.get_one::<bool>("print") {
            if *value {
                ts.show_details();
            }
        }
        // print_track_data(&file_path, matches);

        // super::track::Track::new(file_path, title, album, artist, year, track_number, genre, album_art)
        ts
    }
}
