use std::path::PathBuf;
use structopt::clap::AppSettings;

arg_enum! {
    #[derive(Debug)]
    pub enum OptColor {
        Always,
        Auto,
        Never
    }
}
arg_enum! {
    #[derive(Debug, PartialEq)]
    pub enum OptTime {
        Modified,
        Accessed,
        Created
    }
}
arg_enum! {
    #[derive(Debug)]
    pub enum OptTimeStyle {
        Default,
        Iso,
        LongIso,
        FullIso
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "ls",
    raw(global_settings = "&[AppSettings::ColoredHelp]")
)]
pub struct Opts {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    pub debug: bool,

    #[structopt(flatten)]
    pub filter_ops: FilterOpts,

    #[structopt(flatten)]
    pub display_ops: DisplayOpts,

    /// Files/Directories to show. If left out,
    /// the current working directory is used
    #[structopt(name = "FILES", parse(from_os_str))]
    pub files: Vec<PathBuf>,
}

#[derive(StructOpt, Debug)]
pub struct DisplayOpts {
    /// Display extended file metadata as a table
    #[structopt(short = "l", long = "long")]
    pub long: bool,

    /// When to show colors
    #[structopt(
        long = "color",
        raw(possible_values = "&OptColor::variants()",
            case_insensitive = "true",
            hide_default_value = "true"),
        default_value = "Auto"
    )]
    pub color: OptColor,

    #[structopt(flatten)]
    pub long_options: LongOpts,
}
#[derive(StructOpt, Debug)]
pub struct LongOpts {
    /// List file sizes with binary prefixes
    #[structopt(short = "b", long = "binary")]
    pub binary: bool,

    /// List file sizes in bytes, without any prefixes
    #[structopt(short = "B", long = "bytes")]
    pub bytes: bool,

    /// List each file's group
    #[structopt(short = "g", long = "group")]
    pub group: bool,

    /// Add a header row to each column
    #[structopt(short = "h", long = "header")]
    pub header: bool,

    /// List each file's number of hard links
    #[structopt(short = "H", long = "links")]
    pub links: bool,

    /// List each file's inode number
    #[structopt(short = "i", long = "inode")]
    pub inode: bool,

    /// Limit the depth of recursion
    #[structopt(short = "L", long = "level")]
    pub level: Option<u8>,

    /// Use the modified timestamp field
    #[structopt(short = "m", long = "modified")]
    pub modified: bool,

    /// List each file's number of file system blocks
    #[structopt(short = "S", long = "blocks")]
    pub blocks: bool,

    /// Which timestamp field to list
    #[structopt(
        short = "t", long = "time",
        raw(possible_values = "&OptTime::variants()",
            case_insensitive = "true",
            hide_default_value = "true"),
        default_value = "Modified"
    )]
    pub time: OptTime,

    /// How to format timestamps
    #[structopt(
        long = "time-style",
        raw(possible_values = "&OptTimeStyle::variants()",
            case_insensitive = "true",
            hide_default_value = "true"),
        default_value = "Default"
    )]
    pub time_style: OptTimeStyle,

    /// Use the accessed timestamp field
    #[structopt(short = "u", long = "accessed")]
    pub accessed: bool,

    /// Use the created timestamp field
    #[structopt(short = "U", long = "created")]
    pub created: bool,
}
impl LongOpts {
    pub fn active_count(&self) -> usize {
        let f = |b: bool| { if b { 1 } else { 0 } };

        5 + // Permissions, Size, User, Time and Name are always active
        f(self.group) +
        f(self.links) +
        f(self.inode) +
        f(self.blocks) +
        if self.modified && self.time != OptTime::Modified { 1 } else { 0 } +
        if self.accessed && self.time != OptTime::Accessed { 1 } else { 0 } +
        if self.created  && self.time != OptTime::Created  { 1 } else { 0 }
    }
}

#[derive(StructOpt, Debug)]
pub struct FilterOpts {
    /// Glob patterns, pipe-separated, of files to ignore
    #[structopt(short="I", long="ignore")]
    pub ignore_patterns: Option<String>,
    
    /// Show hidden files
    #[structopt(short = "a", long = "all")]
    pub all: bool,

    /// Reverse sort order
    #[structopt(short = "r", long = "reverse")]
    pub reverse: bool,
}