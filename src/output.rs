pub trait Output: Clone + Send + Sync + 'static {
    fn header(&self);

    fn color(&self, color: OutputColor);
}

#[derive(Debug, Clone, Copy)]
pub struct PpmOutput {
    pub image_width: u32,
    pub image_height: u32,
    pub brightness: u32,
}

impl Output for PpmOutput {
    fn header(&self) {
        println!(
            "P3\n{} {}\n{}",
            self.image_width, self.image_height, self.brightness
        );
    }

    fn color(&self, color: OutputColor) {
        print!("{} {} {} ", color.red, color.green, color.blue);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OutputColor {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl OutputColor {
    pub fn as_bytes(&self) -> [[u8; 4]; 3] {
        [
            self.red.to_le_bytes(),
            self.green.to_le_bytes(),
            self.blue.to_le_bytes(),
        ]
    }
}
