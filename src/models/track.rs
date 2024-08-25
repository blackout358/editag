use std::{clone, path::PathBuf};

use serde::{Deserialize, Serialize};

use id3::TagLike;
use image::ImageReader;
use std::io::Cursor;

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Debug)]
pub struct Track {
    pub tag: Option<id3::Tag>,
    pub file_path: Option<PathBuf>,
    pub title: Option<String>,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub year: Option<i32>,
    pub track_number: Option<i32>,
    pub genre: Option<String>,
    pub album_art: Option<PathBuf>,
    pub show_details: bool,
}

impl Track {
    pub fn new(
        file_path: Option<PathBuf>,
        tag: Option<id3::Tag>,
        title: Option<String>,
        album: Option<String>,
        artist: Option<String>,
        year: Option<i32>,
        track_number: Option<i32>,
        genre: Option<String>,
        album_art: Option<PathBuf>,
    ) -> Self {
        Track {
            file_path,
            tag,
            title,
            album,
            artist,
            year,
            track_number,
            genre,
            album_art,
            show_details: false,
        }
    }
    pub fn show_details(&mut self) {
        self.show_details = true;
    }

    pub fn update_track(&self) {
        if let Some(mut tag) = self.tag.clone() {
            let mut wrote: i32 = 0;
            if let Some(image_path) = &self.album_art {
                let image = ImageReader::open(image_path)
                    .expect("Error opening Album art")
                    .with_guessed_format()
                    .expect("Error guessing format")
                    .decode()
                    .expect("Error decoding image");

                let mut raw_image = Cursor::new(Vec::new());

                image
                    .write_to(&mut raw_image, image::ImageFormat::Jpeg)
                    .unwrap();
                let first_picture = tag.pictures().next();

                tag.remove_all_pictures();
                tag.add_frame(id3::frame::Picture {
                    mime_type: "image/jpeg".to_string(),
                    picture_type: id3::frame::PictureType::CoverFront,
                    description: String::from("Cover Art"),
                    data: raw_image.into_inner(),
                });
                wrote += 1;
                println!("Updated image");
            }

            if let Some(title) = &self.title {
                tag.set_title(title.as_str());
                println!(
                    "Set title successfully: {:?}",
                    tag.title().expect("{Error setting title}")
                );
                wrote += 1;
            }
            if let Some(album) = &self.album {
                tag.set_album(album.as_str());
                println!(
                    "Set album successfully: {:?}",
                    tag.album().expect("{Error setting album}")
                );
                wrote += 1;
            }

            if let Some(artist) = &self.artist {
                tag.set_artist(artist.as_str());
                println!(
                    "Set artist successfully: {:?}",
                    tag.artist().expect("{Error setting artist}")
                );
                wrote += 1;
            }
            if let Some(year) = &self.year {
                tag.set_year(*year);
                println!(
                    "Set year successfully: {:?}",
                    tag.year().expect("{Error setting year}")
                );
                wrote += 1;
            }

            if let Some(genre) = &self.genre {
                tag.set_genre(genre.as_str());
                println!(
                    "Set year successfully: {:?}",
                    tag.genre().expect("{Error setting genre}")
                );
                wrote += 1;
            }

            if let Some(track_number) = &self.track_number {
                tag.set_track(*track_number as u32);
                println!(
                    "Set track number successfully: {:?}",
                    tag.track().expect("{Error setting track number}")
                );
                wrote += 1;

                if wrote > 0 {
                    let _res =
                        tag.write_to_path(self.file_path.as_ref().unwrap(), id3::Version::Id3v24);

                    println!("Wrote tag to file {:?}", self.file_path.as_ref().unwrap());
                }
            }
        }
    }

    pub fn check_dispaly(&self) {
        if self.show_details {
            match &self.tag {
                Some(tag) => {
                    let frames = tag.frames();
                    println!("{}", tag.version());

                    println!(
                        "{0: <8} | {1: <40} | {2: <10}",
                        "Frame ID", "Frame Name", "Frame Content"
                    );
                    println!("{:+>9}|{:+>42}|{:+>20}", "", "", "");
                    for frame in frames {
                        println!(
                            "{0: <8} | {1: <40} | {2: <10}",
                            frame.id(),
                            frame.name(),
                            frame.content()
                        );
                    }
                    println!("\n");
                }
                None => println!("Invalid id3 tag"),
            }
        }
    }
}
