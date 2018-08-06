#![allow(dead_code)]

#[derive(Debug)]
pub struct IgnorePatterns;

#[derive(Debug)]
pub enum SortField {
    Unsorted,
    Name,
    Extension,
    Size,
    Inode,
    ModifiedDate,
    AccessedDate,
    CreatedDate,
    FileType,
    ModifiedAge
}

#[derive(Debug)]
pub struct FileFilter {
    /// Whether to show hidden files (starting with '.') or not
    /// In either case, '.' and '..' won't ever show up,
    /// they are useless imo
    pub show_all: bool,

    // TODO:
    /// not implemented so far
    pub ignore_patterns: IgnorePatterns,

    /// Whether to list all directories in the beginning
    pub list_dirs_first: bool,

    /// Reverses sort order
    pub reverse: bool,

    /// Which property to sort by
    pub sort_field: SortField,
}