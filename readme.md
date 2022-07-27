# FFclip

This piece of code will send your clipboard image to ffmpeg. The desired output is left as an exercise for the user!

# Build

```sh
cargo build --release
```

The executable will be located at `target/release/ffclip`

## Usage

Once an image is in your clipboard (and FFmpeg exists in your PATH env):

```sh
./ffclip .webp
./ffclip image.png
./ffclip -qscale:v 2 capybara.jpg
```

## Acknowledgment

The app is cross-platform thanks to [arboard](https://github.com/1Password/arboard).

## License

[See here](license.md)
