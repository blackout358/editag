use std::{fs, path::PathBuf};

use id3::{TagLike, Timestamp};
use image::ImageReader;
use std::io::Cursor;

use crate::models::controls::{ChangeSet, ModifyAction};

#[derive(Debug)]
pub struct Track {
    pub tag: id3::Tag,
    pub path: PathBuf,
}

impl Track {
    pub fn load(path: PathBuf) -> Result<Self, String> {
        let tag = id3::Tag::read_from_path(&path).map_err(|e| {
            format!(
                "Error occurred when opening ID3 tag :: {}\nWas the correct file path used?",
                e
            )
        })?;
        Ok(Track { path, tag })
    }

    fn set_cover_art(&mut self, img_path: &PathBuf) -> Result<(), String> {
        let img = ImageReader::open(img_path)
            .map_err(|e| format!("Error opening image :: {}", e))?
            .with_guessed_format()
            .map_err(|e| format!("Error guessing format :: {}", e))?
            .decode()
            .map_err(|e| format!("Error decoding image :: {}", e))?;

        let mut raw_image = Cursor::new(Vec::new());

        img.write_to(&mut raw_image, image::ImageFormat::Jpeg)
            .map_err(|e| format!("Error processing image :: {}", e))?;

        self.tag.remove_all_pictures();
        self.tag.add_frame(id3::frame::Picture {
            mime_type: "image/jpeg".to_string(),
            picture_type: id3::frame::PictureType::CoverFront,
            description: "Cover Art".to_string(),
            data: raw_image.into_inner(),
        });

        Ok(())
    }

    pub fn format_filename(&mut self) -> Result<(), String> {
        if let (Some(n), Some(t)) = (self.tag.track(), self.tag.title()) {
            let parent = self
                .path
                .parent()
                .unwrap_or_else(|| std::path::Path::new(""));

            let ext = self
                .path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("mp3");

            let new_name = format!("{:0>2} - {}.{}", n, t, ext);
            let new_path = parent.join(new_name);

            fs::rename(&self.path, &new_path)
                .map_err(|e| format!("Error renaming file :: {}", e))?;
            println!("Successfully formatted file :: {:?}", new_path);
            self.path = new_path;
        }
        Ok(())
    }
    pub fn print_details(&self) {
        let mut frames: Vec<_> = self.tag.frames().collect();
        frames.sort_by(|a, b| a.id().cmp(b.id()));
        println!("{}", self.tag.version());

        println!(
            "{0: <8} | {1: <40} | {2: <10}",
            "Frame ID", "Frame Name", "Frame Content"
        );
        println!("{:->9}+{:->42}+{:->20}", "", "", "");
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

    pub fn save(&self, version: id3::Version) -> Result<(), id3::Error> {
        self.tag.write_to_path(&self.path, version)
    }

    pub fn apply(
        &mut self,
        change_set: &ChangeSet,
        version: id3::Version,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut wrote = 0;
        let total_changes = change_set.actions.len()
            + change_set.delete_all as usize
            + change_set.format_file as usize;

        let mut modified_file = false;
        let mut printed_details = false;

        if change_set.print_details {
            self.print_details();
            printed_details = true
        }

        if change_set.delete_all {
            let tag_clone = self.tag.clone();
            let frames = tag_clone.frames();
            for frame in frames {
                self.tag.remove(frame.id());
                println!("Deleted {}, Containing: {}", frame.id(), frame.content());
            }
            wrote += 1;
        }
        for action in &change_set.actions {
            match action {
                ModifyAction::Title(s) => {
                    self.tag.set_title(s);
                    println!("Set title successfully: {:?}", s);
                    wrote += 1;
                }

                ModifyAction::Album(s) => {
                    self.tag.set_album(s);
                    println!("Set album successfully: {:?}", s);
                    wrote += 1;
                }

                ModifyAction::Artist(s) => {
                    self.tag.set_artist(s);
                    println!("Set artist successfully: {:?}", s);
                    wrote += 1;
                }

                ModifyAction::AlbumArtist(s) => {
                    self.tag.set_album_artist(s);
                    println!("Set album artist successfully: {:?}", s);
                    wrote += 1;
                }

                ModifyAction::Year(y) => {
                    if version == id3::Version::Id3v24 {
                        self.tag.set_date_recorded(Timestamp {
                            year: *y,
                            ..Default::default()
                        });
                    } else {
                        self.tag.set_year(*y);
                    }
                    println!("Set year successfully: {}", y);
                    wrote += 1;
                }

                ModifyAction::Genre(s) => {
                    self.tag.set_genre(s);
                    println!("Set genre successfully: {:?}", s);
                    wrote += 1;
                }

                ModifyAction::TrackNumber(n) => {
                    self.tag.set_track(*n);
                    println!("Set track number successfully: {:?}", n);
                    wrote += 1;
                }

                ModifyAction::DeleteTag(id) => {
                    let removed = self.tag.remove(id);
                    for f in removed {
                        println!("Deleted: {}, Containing: {}", f.id(), f.content());
                    }
                    wrote += 1;
                }

                ModifyAction::CoverArt(p) => {
                    self.set_cover_art(p)?;
                    println!("Updated image");
                    wrote += 1;
                }
            }
        }

        if total_changes == 0 && !printed_details {
            return Err(Box::from("Missing arguments, use 'editag --help' for help"));
        }

        if wrote > 0 {
            modified_file = true;
        }
        Ok(modified_file)
    }
}
