#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// No multithreading
    #[arg(short, long, default_value_t = false)]
    pub single_thread: bool,
}
