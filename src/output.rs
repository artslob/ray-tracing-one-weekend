#[derive(Debug, Clone, Copy)]
pub struct PpmOutput {
    pub image_width: i32,
    pub image_height: i32,
    pub brightness: i32,
}

impl PpmOutput {
    pub fn header(&self) {
        println!(
            "P3\n{} {}\n{}",
            self.image_width, self.image_height, self.brightness
        );
    }

    pub fn color(&self, color: OutputColor) {
        print!("{} {} {} ", color.red, color.green, color.blue);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OutputColor {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}
