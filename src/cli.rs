
use clap::{Parser, ArgGroup};

/// A simple CLI for interacting with movies and TV shows.
#[derive(Parser, Debug)]
#[command(author = "Your Name", version = "1.0", about = "Search and manage your Movie/TV Show collection(Now written in rust)", long_about = None)]
#[command(group(
    ArgGroup::new("history")
        .args(&["cuntinue", "clear_history"])
        .required(false)
))]
pub struct Args {
    /// Continue watching from current history
    #[arg(short, long)]
    pub cuntinue: bool,

    /// Deletes history
    #[arg(long = "clear-history", long = "delete-history")]
    pub clear_history: bool,

    /// Downloads movie or episode that is selected (if no path is provided, it defaults to the current directory)
    #[arg(short='d', long)]
    pub download: Option<Option<String>>,

    /// Edit config file using an editor defined with lobster_editor in the config (\$EDITOR by default)
    #[arg(short, long)]
    pub edit: bool,


    /// Shows image previews during media selection (requires ueberzugpp to be installed to work with fzf)
    #[arg(short, long = "image-preview")]
    pub image_preview: bool,

    /// Outputs the json containing video links, subtitle links, referrers etc. to stdout
    #[arg(short, long)]
    pub json: bool,

    /// Specify the subtitle language (if no language is provided, it defaults to english)
    #[arg(short, long)]
    pub language: Option<Option<String>>,

    /// Use rofi instead of fzf
    #[arg(long, alias = "dmenu", alias = "external-menu")]
    pub rofi: bool,

    /// Specify the provider to watch from (if no provider is provided, it defaults to UpCloud) (currently supported: Upcloud, Vidcloud)
    #[arg(short, long)]
    pub provider: Option<String>,

    /// Specify the video quality (if no quality is provided, it defaults to 1080)
    #[arg(short, long)]
    pub quality: Option<String>,

    /// Suppress the output from mpv when playing a video
    #[arg(long)]
    pub quiet: bool,

    /// Lets you select from the most recent movies or TV shows (if no argument is provided, it defaults to movies)
    #[arg(short, long)]
    pub recent: Option<Option<String>>,

    /// Use Syncplay to watch with friends
    #[arg(short, long)]
    pub syncplay: bool,

    /// Lets you select from the most popular movies and shows
    #[arg(short, long)]
    pub trending: bool,

    /// Update the script
    #[arg(short, long)]
    pub update: bool,


    /// Enable debug mode (prints out debug info to stdout and also saves it to /tmp/lobster.log)
    #[arg(short='x', long)]
    pub debug: bool,

    /// The query to search for a Movie/TV Show
    #[arg(value_name = "query")]
    pub query: Option<String>,
}

