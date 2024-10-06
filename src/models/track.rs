use std::{fs, path::PathBuf};

use id3::{TagLike, Timestamp};
use image::ImageReader;
use std::io::Cursor;

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
    pub to_delete: Option<String>,
    pub delete_all: bool,
    pub show_details: bool,
    pub format_file: bool,
    pub version: id3::Version,
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
        to_delete: Option<String>,
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
            to_delete,
            show_details: false,
            delete_all: false,
            format_file: false,
            version: id3::Version::Id3v24,
        }
    }
    pub fn show_details(&mut self) {
        self.show_details = true;
    }

    pub fn delete_all(&mut self) {
        self.delete_all = true;
    }

    pub fn format_file(&mut self) {
        self.format_file = true;
    }

    pub fn set_path(&mut self, new_path: Option<PathBuf>) {
        self.file_path = new_path;
    }

    pub fn update_track(&mut self) {
        if let Some(mut tag) = self.tag.clone() {
            let mut wrote: i32 = 0;

            if self.format_file {
                let tn = tag.track();
                let tt = tag.title();

                if let (Some(n), Some(t)) = (tn, tt) {
                    let p = self.file_path.as_ref().unwrap();
                    let str_p = p.to_str().unwrap();
                    let index = str_p.rfind('/').unwrap();
                    let file_format = str_p.rfind('.').unwrap();
                    let new = format!(
                        "{}/{:0>2} - {}{}",
                        &str_p[..index],
                        n,
                        t,
                        &str_p[file_format..]
                    );

                    let _res = fs::rename(p, new.clone());
                    self.set_path(Some(PathBuf::from(&new)));
                    println!("Successfully formatted file :: {}", new);
                } else {
                    if tn == None {
                        println!("Missing track number");
                    }
                    if tt == None {
                        println!("Missing track title");
                    }
                    println!();
                }
            }

            if self.delete_all {
                let tag_clone = tag.clone();
                let frames = tag_clone.frames();
                for frame in frames {
                    tag.remove(frame.id());
                    println!("DELETED {}", frame.id());
                }
                wrote += 1;
            }

            if let Some(image_path) = &self.album_art {
                let image = process_image(image_path);

                match image {
                    Ok(image) => {
                        let mut raw_image = Cursor::new(Vec::new());

                        image
                            .write_to(&mut raw_image, image::ImageFormat::Jpeg)
                            .unwrap();

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
                    Err(err) => println!("Error setting album art :: {}", err),
                }
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
                if self.version == id3::Version::Id3v24 {
                    tag.set_date_recorded(Timestamp { year: *year, month: None, day: None, hour: None, minute: None, second: None });
                    println!(
                        "Set year successfully: {:?}",
                        tag.date_recorded().expect("{Error getting set year}").year
                    );
                } else {
                    tag.set_year(*year);
                    println!(
                        "Set year successfully: {:?}",
                        tag.year().expect("{Error getting set year}")
                    );
                }
                wrote += 1;
            }

            if let Some(genre) = &self.genre {
                tag.set_genre(genre.as_str());
                println!(
                    "Set genre successfully: {:?}",
                    tag.genre().expect("{Error getting set genre}")
                );
                wrote += 1;
            }

            if let Some(track_number) = &self.track_number {
                tag.set_track(*track_number as u32);
                println!(
                    "Set track number successfully: {:?}",
                    tag.track().expect("{Error getting set track number}")
                );
                wrote += 1;
            }
            if let Some(to_delete) = &self.to_delete {
                let frames = tag.remove(to_delete);

                for frame in frames {
                    println!("Deleted: {}, Containing: {}", frame.id(), frame.content());
                }
                wrote += 1;
            }

            if wrote > 0 {
                let _res = tag.write_to_path(self.file_path.as_ref().unwrap(), self.version);

                match _res {
                    Ok(_) => println!("Wrote tag to file {:?}", self.file_path.as_ref().unwrap()),
                    Err(e) => println!("Error saving tag to file :: {}", e),
                }
            } else {
                println!("Missing arguments, use 'editag --help' for help ");
            }
        } else {
            println!("Invalid file, use 'editag --help' for help  ");
        }
    }

    pub fn check_display(&self) {
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

fn process_image(image_path: &PathBuf) -> Result<image::DynamicImage, image::ImageError> {
    let image = ImageReader::open(image_path);
    match image {
        Ok(im) => {
            let fo = im.with_guessed_format();
            match fo {
                Ok(de) => {
                    let out = de.decode();
                    return out;
                }
                Err(err) => {
                    println!("Error guessing format");
                    return Err(image::ImageError::IoError(err));
                }
            }
        }
        Err(e) => {
            println!("Error opening image :: {}", e);
            return Err(image::ImageError::IoError(e));
        }
    }
}
