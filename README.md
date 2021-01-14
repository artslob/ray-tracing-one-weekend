# ray tracing on one weekend

### Usage
```bash
cargo run --release > image.ppm; convert image.ppm image.png
```

### Notes
To convert all `.ppm` images to `.png`:
```bash
find . -iname "*.ppm" -exec sh -c 'convert {} $(basename {} .ppm).png' \;
```
