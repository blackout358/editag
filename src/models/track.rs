use std::path::{Path, PathBuf};

use audiotags::{Picture, Tag};
use serde::{Deserialize, Serialize};

use id3::TagLike;
use image::ImageReader;
use std::io::Cursor;

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    pub file_path: Option<PathBuf>,
    pub title: Option<String>,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub year: Option<i32>,
    pub track_number: Option<i32>,
    pub genre: Option<String>,
    pub album_art: Option<PathBuf>,
}

impl Track {
    pub fn new(
        file_path: Option<PathBuf>,
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
            title,
            album,
            artist,
            year,
            track_number,
            genre,
            album_art,
        }
    }

    pub fn update_track(&self) {
        // let mut tag = id3::Tag::read_from_path(self.file_path).expect("Error reading mp3 tag");

        match &self.file_path {
            Some(path) => {
                let mut tag = id3::Tag::read_from_path(path).expect("Error reading mp3 tag");
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

                    tag.add_frame(id3::frame::Picture {
                        mime_type: "image/jpeg".to_string(),
                        picture_type: id3::frame::PictureType::CoverFront,
                        description: String::new(),
                        data: raw_image.into_inner(),
                    });

                    let res = tag.write_to_path(path, id3::Version::Id3v23);
                    println!("{:?}", res);
                }
                if let Some(title) = &self.title {
                    tag.set_title(title.as_str());
                }
                if let Some(album) = &self.album {
                    tag.set_album(album.as_str());
                }

                if let Some(artist) = &self.artist {
                    tag.set_artist(artist.as_str());
                }
                if let Some(year) = &self.year {
                    tag.set_year(*year);
                }

                if let Some(genre) = &self.genre {
                    tag.set_genre(genre.as_str());
                }

                if let Some(track_number) = &self.track_number {
                    tag.set_track(*track_number as u32);
                }
                let res = tag.write_to_path(path, id3::Version::Id3v23);
                println!("{:?}", res);
            }
            None => {}
        }

        match &self.file_path {
            Some(path) => {
                let mut track = Tag::new().read_from_path(path).unwrap();
                if let Some(title) = &self.title {
                    track.set_title(title.as_str());
                }
                if let Some(album) = &self.album {
                    track.set_album_title(album.as_str());
                }

                if let Some(artist) = &self.artist {
                    track.set_artist(artist.as_str());
                }
                if let Some(year) = &self.year {
                    track.set_year(*year);
                }

                if let Some(genre) = &self.genre {
                    track.set_genre(genre.as_str());
                }
                if let Some(track_number) = &self.track_number {
                    track.set_track_number(*track_number as u16);
                }

                if let Some(track_number) = &self.track_number {
                    track.set_track_number(*track_number as u16);
                }

                // if let Some(image) = &self.album_art {
                //     println!("valid path");
                //     let img = ImageReader::open(image)
                //         .expect("Error opening Album art")
                //         .with_guessed_format()
                //         .expect("Error guessing format")
                //         .decode()
                //         .expect("Error decoding image");

                //     let rgb_image = img.as_rgb8().expect("Error converting to RGB8");

                //     println!("Converted to rgb8");

                //     let raw_image = rgb_image.as_raw();

                //     println!("Got raw image");
                //     // let ts = img.as_flat_samples_u8().unwrap();
                //     // let asd = img
                //     //     .as_rgba8()
                //     //     .expect("Failed to convert image to rgba8 vector");
                //     // let ts = img.fl
                //     // ts.
                //     // println!("Raw vector {:?}", raw_image);
                //     let pic = Picture {
                //         mime_type: audiotags::MimeType::Jpeg,
                //         data: raw_image,
                //     };

                //     // audiotags::Id3v2Tag::new()

                //     let asd = track.album_cover().expect("Cannot read image of track");

                //     println!("{:?}", asd);

                //     // println!("{:?}", pic);

                //     println!("Created Picture");
                //     // track.
                //     track.set_album_cover(pic);
                //     println!("Set album cover");
                //     // assert_eq!(track.album_cover(), Some(pic));
                //     // track.remove_album_cover();
                // }
                track
                    .write_to_path(path.to_str().unwrap())
                    .expect("Failed to save");
                println!("Updated Track");
                // println!("{:#?}", track);
            }
            None => todo!(),
        }
    }
}
