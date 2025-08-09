#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// No multithreading
    #[arg(short, long, default_value_t = false)]
    pub single_thread: bool,
    #[arg(long, default_value_t = 500)]
    pub samples_per_pixel: u32,
    #[arg(long, default_value_t = 50)]
    pub max_depth: u32,
}
