# Editag

ID3 editor for mp3 files written in Rust

use `cargo build --release` to compile

```
Usage: editag [OPTIONS] [file_path]

Arguments:
  [file_path]

Options:
  -t, --title <title>                Set the track title
  -a, --album <album>                Set the album
  -A, --artist <artist>              Set the artist
  -B, --album-artist <album-artist>  Set the album artist
  -y, --year <year>                  Set the year
  -n, --track-number <track_number>  Set the track number
  -g, --genre <genre>                Set the genre
  -c, --cover-art <path_to_image>    Set the cover art

  -p, --print-data                   View files existing id3 data
      --list-genres                  List ID3 genres (ignores other flags)
  -d, --delete-tag <delete-tag>      Delete a frame by passing in its id
  -D, --delete-all-tags              Delete all frames
  -r                                 Run command on every mp3 file in the specified directory or current directory if none is provided
  -f, --format-file                  Format file to 'TN - TRACK TITLE' TN = track number

      --v23                          Attempts to save tag as ID3v2.3 instead of ID3v2.4
      --v22                          Attempts to save tag as ID3v2.2 instead of ID3v2.4

  -C, --custom <Frame id> <Value>    Set a custom frame and its value
  -h, --help                         Print help
  -V, --version                      Print version
```

## Examples

### Setting basic metadata

```bash
$ editag Love\ Trip/01\ -\ Love\ Trip.mp3 -y 1982 -A "Takako Mamiya" -a "Love Trip" -n 1 -g "City Pop"
Set artist successfully: "Takako Mamiya"
Set album successfully: "Love Trip"
Set genre successfully: "City Pop"
Set year successfully: 1982
Set track number successfully: 1
Processed: "Love Trip/01 - Love Trip.mp3"

$ editag Love\ Trip/01\ -\ Love\ Trip.mp3 -p
ID3v2.4
Frame ID | Frame Name                               | Frame Content
---------+------------------------------------------+--------------------
TALB     | Album/Movie/Show title                   | Love Trip
TCON     | Content type                             | City Pop
TDRC     | Recording time                           | 1982
TPE1     | Lead performer(s)/Soloist(s)             | Takako Mamiya
TRCK     | Track number/Position in set             | 1


Processed: "Love Trip/01 - Love Trip.mp3"
```

### Setting the cover art

```bash
$ editag Love\ Trip/01\ -\ Love\ Trip.mp3 -c Love\ Trip/27865162756.jpg
Cover Art Love Trip/27865162756.jpg
Updated image
Processed: "Love Trip/01 - Love Trip.mp3"

$ editag -p Love\ Trip/01\ -\ Love\ Trip.mp3
ID3v2.4
Frame ID | Frame Name                               | Frame Content
---------+------------------------------------------+--------------------
APIC     | Attached picture                         | Cover Art: Front cover (image/jpeg, 155991 bytes)
TALB     | Album/Movie/Show title                   | Love Trip
TCON     | Content type                             | City Pop
TDRC     | Recording time                           | 1982
TPE1     | Lead performer(s)/Soloist(s)             | Takako Mamiya
TRCK     | Track number/Position in set             | 1
```

### Setting a custom tag

```bash
$ editag -C TIT3 "Frogs are kind of cool I guess" Love\ Trip/01\ -\ Love\ Trip.mp3
No tag found for "Love Trip/01 - Love Trip.mp3", creating a new one
Saved
Processed: "Love Trip/01 - Love Trip.mp3"

$ editag -p Love\ Trip/01\ -\ Love\ Trip.mp3 ID3v2.4
Frame ID | Frame Name                               | Frame Content
---------+------------------------------------------+--------------------
TIT3     | Subtitle/Description refinement          | Frogs are kind of cool I guess
```

This ONLY supports headers that expect raw text. Anything that expects structured data like COMMS
will not accept this and the previous header will be destroyed. There is currently no backup or
copy write implemented in editag.

```bash
$ editag -C COMM "Frogs are kind of cool I guess" Love\ Trip/01\ -\ Love\ Trip.mp3
Failed the save "Love Trip/01 - Love Trip.mp3": InvalidInput: Frame with ID COMM and content type Text can not be written as valid ID3
Saved
Processed: "Love Trip/01 - Love Trip.mp3"

$ editag -p Love\ Trip/01\ -\ Love\ Trip.mp3
No tag found for "Love Trip/01 - Love Trip.mp3", creating a new one
ID3v2.4
Frame ID | Frame Name                               | Frame Content
---------+------------------------------------------+--------------------


Processed: "Love Trip/01 - Love Trip.mp3"

```
