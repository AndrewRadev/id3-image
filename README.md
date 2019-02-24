This tool wraps the [rust-id3](https://github.com/jameshurst/rust-id3) library to provide an easy way to manipulate embedded cover art in an mp3 file. It was made for my own personal usage, so it likely makes assumptions that are not ideal for all cases. If it doesn't work for you, please open [a github issue](https://github.com/AndrewRadev/id3-image/issues).

It comes with three executables:

- `id3-image-embed`
- `id3-image-extract`
- `id3-image-remove`

A summary of their usage follows. You can also execute them with `--help` for additional information.

## Embedding

To embed an image into an mp3 file, use `id3-image-embed`:

```
id3-image-embed <music-file.mp3> <image-file.jpg>
```

The image doesn't *have* to be a JPEG, anything that the [image](https://github.com/PistonDevelopers/image) library accepts will work. It will, however, be embedded as a JPEG for no particular reason other than seeming like a sensible default choice.

The ID3 tags will be stored as ID3v2.3, and written in-place in the music file.

## Extracting

In order to extract an embedded image file from an mp3, use `id3-image-extract`:

```
id3-image-extract <music-file.mp3> [image-file.jpg]
```

The image filepath is optional -- if omitted, the image will be named the same as the music file with a `jpg` extension. If an image file with that name already exists, it'll silently be overwritten. If the song file has no embedded images, you'll get an error message.

You can append a `-v` or `--verbose` flag to echo the image filename to stdout for scripting purposes.

## Removing

Remove all embedded images from an mp3 file using `id3-image-remove`:

```
id3-image-remove <music-file.mp3>
```

This might be useful if you *don't* like cover art and would like to "clean" your mp3 files. The tool will ask for confirmation before deletion, just in case it was invoked by mistake. You can append `--no-confirm` to avoid this, if you're using it for scripting purposes.

## Bonus scripts

You can use `id3-image-extract` to build a "preview" tool for a quick look at an image's cover art. All you need is a temporary filename to extract to:

``` bash
#! /bin/sh

set -e

if [ $# -lt 1 ]; then
  echo "USAGE: id3-image-preview <music-file.mp3>"
  exit 1
fi

music_file=$1

# Create an auto-removed file when script exits
image_file=$(mktemp --suffix='.jpg')
exec 3>"$image_file"
rm "$image_file"

id3-image-extract "$music_file" "$image_file"
// Would probably work with `open` on a mac, but hasn't been tested:
xdg-open "$image_file"
```

You can also make a script that chooses the image to embed using a GUI file picker. You'll need `zenity` for this one:

``` bash
#! /bin/sh

set -e

if [ $# -lt 1 ]; then
  echo "USAGE: id3-image-gui-embed <music-file.mp3>"
  exit 1
fi

music_file=$1

# Pick the image file with a file chooser
image_file=$(zenity --file-selection --filename="$(pwd)/" --title="Select Cover Art")

id3-image-embed "$music_file" "$image_file"
```

This particular popup doesn't filter by images-only, because I couldn't figure out how to make a generic "image" filter with zenity. You can append a `--file-filter='*.jpg'` flag to show only JPEGs.

These scripts can be particularly convenient when attached to a GUI file manager. My preferred one, [Thunar](https://docs.xfce.org/xfce/thunar/start), allows "Custom actions" that these two can be plugged in.

## Music used for testing:

Elevator Music Attempt 1 by Christian Bakker: http://www.jamendo.com/en/list/a98147/echoes-from-the-past

## Possible future work

- Don't silently overwrite images when extracting
- Allow different output formats
- Allow different ID3 versions
- Allow embedding/extracting multiple images (currently, on the first one is extracted)
