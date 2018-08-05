use std::path::PathBuf;
use structopt::clap::AppSettings;

arg_enum! {
    #[derive(Debug)]
    pub enum ColorOption {
        Always,
        Auto,
        Never
    }
}

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(
    name = "ls",
    raw(global_settings = "&[AppSettings::ColoredHelp]")
)]
pub struct Opts {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    pub debug: bool,

    // The number of occurences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: u8,

    /// Show hidden files
    #[structopt(short = "a", long = "all")]
    pub all: bool,

    /// Display extended file metadata as a table
    #[structopt(short = "l", long = "long")]
    pub long: bool,

    /// When to show colors
    #[structopt(
        long = "color",
        raw(
            possible_values = "&ColorOption::variants()",
            case_insensitive = "true",
            hide_default_value = "true"),
        default_value = "Auto"
    )]
    pub color: ColorOption,

    /// Glob patterns, pipe-separated, of files to ignore
    #[structopt(short="I", long="ignore")]
    pub ignore_patterns: Option<String>,

    /// Files/Directories to process. If left out,
    /// the current working directory is used
    #[structopt(name = "FILES", parse(from_os_str))]
    pub files: Vec<PathBuf>,
}