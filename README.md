ID3 editor for mp3 files written in Rust

```
Usage: editag [OPTIONS] [file_path]

Arguments:
  [file_path]  

Options:
  -t, --title <title>                Set the track title
  -a, --album <album>                Set the album
  -A, --artist <artist>              Set the artist
  -y, --year <year>                  Set the year
  -n, --track-number <track_number>  Set the track number
  -g, --genre <genre>                Set the genre
  -c, --cover-art <path_to_image>    Set the cover art
                                     
  -p, --print-data                   View files existing id3 data
      --list-genres                  Lists genres and their associated number for ID3
  -d, --delete-tag <tag-to-delete>   Delete a frame by passing in its id
  -D, --delete-all-tags              Delete all frames
  -f, --format-file                  Format file to 'TN - TRACK TITLE' TN = track number
                                     
      --v23                          Attempts to save tag as ID3v2.3 instead of ID3v2.4
      --v22                          Attempts to save tag as ID3v2.2 instead of ID3v2.4
                                     
  -h, --help                         Print help
  -V, --version                      Print version

```
