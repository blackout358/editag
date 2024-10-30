mod models;
use models::parser::MyParser;

fn main() {
    let parser = MyParser::generate_commandline_args();
    let mut music_track = MyParser::parse_command(&parser);

    if let Some(_) = music_track.recursive {
        let tracks = MyParser::get_mp3s_in_dir(&music_track);
        for track in tracks {
            music_track.file_path = Some(track);
            music_track.update_tag();
            music_track.update_track();
        }
    } else {
        music_track.update_track();
    }
}
