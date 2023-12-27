# pixie

Pixelate images in Rust

## Usage

Generate a pixelated JPEG image

```bash
cargo run <file_path> <pixel_width> <pixel_height>
cargo run ./j.jpg 10 10
```

Generate a series of pixelated JPEG images and merge them into a GIF animation

```bash
chmod +x pixie.sh
./pixie.sh
```
