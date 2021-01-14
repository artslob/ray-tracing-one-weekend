# ray tracing on one weekend

### Usage
```bash
cargo run --release > image.ppm; convert image.ppm image.png
```

### About

My final render:
![13.1 Final Render](images/13.1.png?raw=true "13.1 Final Render")

### Notes
To convert all `.ppm` images to `.png`:
```bash
find . -iname "*.ppm" -exec sh -c 'convert {} $(basename {} .ppm).png' \;
```
