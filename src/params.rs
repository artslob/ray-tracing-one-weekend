#[derive(Debug, Clone)]
pub struct Params {
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

impl From<&crate::cli::Args> for Params {
    fn from(args: &crate::cli::Args) -> Self {
        Self {
            samples_per_pixel: args.samples_per_pixel,
            max_depth: args.max_depth,
        }
    }
}
