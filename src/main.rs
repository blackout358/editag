mod models;
use models::parser::MyParser;

fn main() {
    let parser = MyParser::generate_commandline_args();
    let mut music_track = MyParser::parse_command(&parser);

    if music_track.0.recursive {
        let tracks = MyParser::get_mp3s_in_dir();
        for track in tracks {
            music_track.0.file_path = Some(track);
            music_track.0.update_tag();
            music_track.0.update_track(music_track.1);
        }
    } else {
        music_track.0.update_track(music_track.1);
    }
}
