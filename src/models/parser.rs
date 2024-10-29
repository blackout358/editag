use std::{fs, path::PathBuf};

use clap::{command, Arg, ArgAction, ArgMatches};
use regex::Regex;

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
                Arg::new("list-genres")
                    .long("list-genres")
                    .help("Lists genres and their associated number for ID3")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("tag-to-delete")
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
                    .long("recursive")
                    .help("Run command to every mp3 file in the current directory")
                    .action(ArgAction::SetTrue),
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
            .get_matches();
        matches
    }

    pub fn parse_command(matches: &ArgMatches) -> super::track::Track {
        let mut tag: Option<id3::Tag> = None;
        let file_path: Option<PathBuf> = match matches.get_one::<String>("file_path") {
            Some(name) => {
                let path: PathBuf = [r"./", name].iter().collect();
                match id3::Tag::read_from_path(&path) {
                    Ok(t) => tag = Some(t),
                    Err(e) => println!("Error occured when opening id3 tag :: {}", e),
                }
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
            matches.get_one::<String>("tag-to-delete").cloned(),
            image_path,
        );

        if let Some(value) = matches.get_one::<bool>("print") {
            if *value {
                ts.show_details();
            }
        }
        if let Some(value) = matches.get_one::<bool>("delete-all") {
            if *value {
                ts.delete_all();
            }
        }

        if let Some(value) = matches.get_one::<bool>("recursive") {
            if *value {
                ts.set_recursive();
            }
        }

        if let Some(value) = matches.get_one::<bool>("format-file") {
            if *value {
                ts.format_file();
            }
        }

        if let Some(value) = matches.get_one::<bool>("v22") {
            if *value {
                ts.version = id3::Version::Id3v22;
            }
        }

        if let Some(value) = matches.get_one::<bool>("v23") {
            if *value {
                ts.version = id3::Version::Id3v23;
            }
        }

        if let Some(value) = matches.get_one::<bool>("list-genres") {
            if *value {
                let genres = vec![
                    "0: Blues",
                    "1: Classic Rock",
                    "2: Country",
                    "3: Dance",
                    "4: Disco",
                    "5: Funk",
                    "6: Grunge",
                    "7: Hip-Hop",
                    "8: Jazz",
                    "9: Metal",
                    "10: New Age",
                    "11: Oldies",
                    "12: Other",
                    "13: Pop",
                    "14: R&B",
                    "15: Rap",
                    "16: Reggae",
                    "17: Rock",
                    "18: Techno",
                    "19: Industrial",
                    "20: Alternative",
                    "21: Ska",
                    "22: Death Metal",
                    "23: Pranks",
                    "24: Soundtrack",
                    "25: Euro-Techno",
                    "26: Ambient",
                    "27: Trip-Hop",
                    "28: Vocal",
                    "29: Jazz+Funk",
                    "30: Fusion",
                    "31: Trance",
                    "32: Classical",
                    "33: Instrumental",
                    "34: Acid",
                    "35: House",
                    "36: Game",
                    "37: Sound Clip",
                    "38: Gospel",
                    "39: Noise",
                    "40: AlternRock",
                    "41: Bass",
                    "42: Soul",
                    "43: Punk",
                    "44: Space",
                    "45: Meditative",
                    "46: Instrumental Pop",
                    "47: Instrumental Rock",
                    "48: Ethnic",
                    "49: Gothic",
                    "50: Darkwave",
                    "51: Techno-Industrial",
                    "52: Electronic",
                    "53: Pop-Folk",
                    "54: Eurodance",
                    "55: Dream",
                    "56: Southern Rock",
                    "57: Comedy",
                    "58: Cult",
                    "59: Gangsta Rap",
                    "60: Top 40",
                    "61: Christian Rap",
                    "62: Pop / Funk",
                    "63: Jungle",
                    "64: Native American",
                    "65: Cabaret",
                    "66: New Wave",
                    "67: Psychedelic",
                    "68: Rave",
                    "69: Showtunes",
                    "70: Trailer",
                    "71: Lo-Fi",
                    "72: Tribal",
                    "73: Acid Punk",
                    "74: Acid Jazz",
                    "75: Polka",
                    "76: Retro",
                    "77: Musical",
                    "78: Rock & Roll",
                    "79: Hard Rock",
                    "80: Folk",
                    "81: Folk-Rock",
                    "82: National Folk",
                    "83: Swing",
                    "84: Fast Fusion",
                    "85: Bebob",
                    "86: Latin",
                    "87: Revival",
                    "88: Celtic",
                    "89: Bluegrass",
                    "90: Avantgarde",
                    "91: Gothic Rock",
                    "92: Progressive Rock",
                    "93: Psychedelic Rock",
                    "94: Symphonic Rock",
                    "95: Slow Rock",
                    "96: Big Band",
                    "97: Chorus",
                    "98: Easy Listening",
                    "99: Acoustic",
                    "100: Humour",
                    "101: Speech",
                    "102: Chanson",
                    "103: Opera",
                    "104: Chamber Music",
                    "105: Sonata",
                    "106: Symphony",
                    "107: Booty Bass",
                    "108: Primus",
                    "109: Porn Groove",
                    "110: Satire",
                    "111: Slow Jam",
                    "112: Club",
                    "113: Tango",
                    "114: Samba",
                    "115: Folklore",
                    "116: Ballad",
                    "117: Power Ballad",
                    "118: Rhythmic Soul",
                    "119: Freestyle",
                    "120: Duet",
                    "121: Punk Rock",
                    "122: Drum Solo",
                    "123: A Cappella",
                    "124: Euro-House",
                    "125: Dance Hall",
                    "126: Goa",
                    "127: Drum & Bass",
                    "128: Club-House",
                    "129: Hardcore",
                    "130: Terror",
                    "131: Indie",
                    "132: BritPop",
                    "133: Negerpunk",
                    "134: Polsk Punk",
                    "135: Beat",
                    "136: Christian Gangsta Rap",
                    "137: Heavy Metal",
                    "138: Black Metal",
                    "139: Crossover",
                    "140: Contemporary Christian",
                    "141: Christian Rock",
                    "142: Merengue",
                    "143: Salsa",
                    "144: Thrash Metal",
                    "145: Anime",
                    "146: JPop",
                    "147: Synthpop",
                    "148: Abstract",
                    "149: Art Rock",
                    "150: Baroque",
                    "151: Bhangra",
                    "152: Big Beat",
                    "153: Breakbeat",
                    "154: Chillout",
                    "155: Downtempo",
                    "156: Dub",
                    "157: EBM",
                    "158: Eclectic",
                    "159: Electro",
                    "160: Electroclash",
                    "161: Emo",
                    "162: Experimental",
                    "163: Garage",
                    "164: Global",
                    "165: IDM",
                    "166: Illbient",
                    "167: Industro-Goth",
                    "168: Jam Band",
                    "169: Krautrock",
                    "170: Leftfield",
                    "171: Lounge",
                    "172: Math Rock",
                    "173: New Romantic",
                    "174: Nu-Breakz",
                    "175: Post-Punk",
                    "176: Post-Rock",
                    "177: Psytrance",
                    "178: Shoegaze",
                    "179: Space Rock",
                    "180: Trop Rock",
                    "181: World Music",
                    "182: Neoclassical",
                    "183: Audiobook",
                    "184: Audio Theatre",
                    "185: Neue Deutsche Welle",
                    "186: Podcast",
                    "187: Indie Rock",
                    "188: G-Funk",
                    "189: Dubstep",
                    "190: Garage Rock",
                    "191: Psybient",
                ];
                let offset = 191 / 2 + 1;
                for (pos, _) in genres[..offset].iter().enumerate() {
                    let mut str = String::new();
                    if pos < offset {
                        str.push_str(format!("{0: <25}", genres[pos]).as_str());
                    }
                    if pos * 2 < genres.len() {
                        str.push_str(format!("{0: <25}", genres[pos + offset]).as_str());
                    }
                    println!("{}", str);
                }
            }
        }

        ts
    }

    pub fn get_mp3s_in_dir() -> Vec<PathBuf> {
        let paths = fs::read_dir("./").unwrap().filter(|s| {
            let filename = s.as_ref().unwrap().path();
            let regex = Regex::new(r"(.*\.mp3)").unwrap();

            if regex.is_match(&filename.to_str().unwrap()) {
                true
            } else {
                false
            }
        });

        let mut filepaths_buffer_vec: Vec<PathBuf> = Vec::new();
        paths.for_each(|p| filepaths_buffer_vec.push(p.unwrap().path()));
        filepaths_buffer_vec
    }
}
