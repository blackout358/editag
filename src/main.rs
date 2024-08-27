mod models;
use models::parser::MyParser;

fn main() {
    let parser = MyParser::generate_commandline_args();
    let mut music_track = MyParser::parse_command(&parser);
    music_track.check_dispaly();
    music_track.update_track();
}
