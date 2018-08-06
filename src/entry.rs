use std::fs;
use std::os::unix::fs::{MetadataExt, FileTypeExt, PermissionsExt};
use std::fmt::{self, Display};
use chrono::{DateTime, Local};
use users::{self, User, Group};
use std::path::PathBuf;
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};

use super::Error;

#[derive(Debug)]
pub enum FileType {
    File,
    Dir,
    Symlink,
    BlockDevice,
    CharDevice,
    Pipe,
    Socket,
    Unknown
}
impl From<fs::FileType> for FileType {
    fn from(ft: fs::FileType) -> Self {
        if ft.is_file() { FileType::File }
        else if ft.is_dir() { FileType::Dir }
        else if ft.is_symlink() { FileType::Symlink }
        else if ft.is_block_device() { FileType::BlockDevice }
        else if ft.is_char_device() { FileType::CharDevice }
        else if ft.is_fifo() { FileType::Pipe }
        else if ft.is_socket() { FileType::Socket }
        else { FileType::Unknown }
    }
}
impl Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            FileType::File        => "-",
            FileType::Dir         => "d",
            FileType::Symlink     => "l",
            FileType::BlockDevice => "b",
            FileType::CharDevice  => "c",
            FileType::Pipe        => "p",
            FileType::Socket      => "s",
            FileType::Unknown     => " ",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, Clone)]
pub struct PermissionTriad {
    pub r: bool,
    pub w: bool,
    pub x: bool,
}
impl PermissionTriad {
    pub fn parse(mode: u32, read: u32, write: u32, execute: u32) -> Self {
        let r = (mode & read) != 0;
        let w = (mode & write) != 0;
        let x = (mode & execute) != 0;
        PermissionTriad { r, w, x }
    }
}
impl Display for PermissionTriad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = if self.r { "r" } else { "-" };
        let w = if self.w { "w" } else { "-" };
        let x = if self.x { "x" } else { "-" };
        write!(f, "{}{}{}", r, w, x)
    }
}
#[derive(Debug, Clone)]
pub struct Permissions {
    pub owner : PermissionTriad,
    pub group : PermissionTriad,
    pub others: PermissionTriad,
}
impl Permissions {
    pub fn from_file_mode(mode: u32) -> Self {
        let owner  = PermissionTriad::parse(mode, S_IRUSR, S_IWUSR, S_IXUSR);
        let group  = PermissionTriad::parse(mode, S_IRGRP, S_IWGRP, S_IXGRP);
        let others = PermissionTriad::parse(mode, S_IROTH, S_IWOTH, S_IXOTH);

        Permissions { owner, group, others }
    }
}
impl Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.owner, self.group, self.others)
    }
}

pub struct Entry {
    pub name: String,
    #[allow(dead_code)]
    pub metadata: fs::Metadata,
    #[allow(dead_code)]
    pub path: PathBuf,
    pub file_type: FileType,
    pub permissions: Permissions,
    pub size: u64,
    pub modified: DateTime<Local>,
    pub num_hardlinks: u64,
    pub user: User,
    pub group: Group,
}
impl Entry {
    pub fn from_dir_entry(de: &fs::DirEntry) -> Result<Self, Error> {
        let metadata: fs::Metadata = de.metadata()?;
        let name: String = de.file_name().to_str().unwrap_or("???").into();
        let path: PathBuf = de.path();
        let file_type = FileType::from(metadata.file_type());
        let permissions = Permissions::from_file_mode(metadata.permissions().mode());
        let size: u64 = metadata.len();
        let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
        let num_hardlinks: u64 = metadata.nlink();
        let user : User  = users::get_user_by_uid (metadata.uid())
            .ok_or(Error::ParseIdError)?;
        let group: Group = users::get_group_by_gid(metadata.gid())
            .ok_or(Error::ParseIdError)?;

        Ok(Entry {
            metadata, name, path, file_type, permissions, size,
            modified, num_hardlinks, user, group
        })
    }
}
impl Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let user_name : String = self.user .name().into();
        let group_name: String = self.group.name().into();

        write!(f, "{}{} {} {} {} {:>5} {} {}",
            self.file_type,
            self.permissions,
            self.num_hardlinks,
            user_name,
            group_name,
            self.size,
            self.modified.format("%_d %b %H:%M"),
            self.name
        )
    }
}