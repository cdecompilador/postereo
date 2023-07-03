use std::path::PathBuf;

/// Configurations loaded from the cli or .env
#[derive(clap::Parser)]
pub struct Config {
    /// Folder where the frontend dist is located
    #[clap(long, env)]
    pub www_dir: PathBuf,
    
    /// Port where to serve the web app
    #[clap(long, env)]
    pub port: u16,

    /// Url to the sqlite database
    #[clap(long, env)]
    pub database_url: String,
}
