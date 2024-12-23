use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct OpenFlags: u32 {
        const READ_ONLY  = 0x0001;
        const WRITE_ONLY = 0x0002;
        const READ_WRITE = 0x0004;
        const CREATE     = 0x0008;
        const TRUNC      = 0x0010;
        const APPEND     = 0x0020;
        const EXCL       = 0x0040;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FileMode: u32 {
        const USER_READ    = 0x0100;
        const USER_WRITE   = 0x0080;
        const USER_EXEC    = 0x0040;
        const GROUP_READ   = 0x0020;
        const GROUP_WRITE  = 0x0010;
        const GROUP_EXEC   = 0x0008;
        const OTHER_READ   = 0x0004;
        const OTHER_WRITE  = 0x0002;
        const OTHER_EXEC   = 0x0001;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeekFrom {
    Start(i64),
    Current(i64),
    End(i64),
}

pub struct FileStat {
    pub size: u64,
    pub mode: FileMode,
    pub uid: u32,
    pub gid: u32,
    pub atime: u64,
    pub mtime: u64,
    pub ctime: u64,
}

pub type FileDesc = i32;

#[derive(Debug)]
pub enum Error {}

pub trait VirtualFileSystem {
    /// Opens a file at the given `path` with the specified flags and mode.
    fn open(
        &self,
        path: &str,
        flags: OpenFlags,
        mode: FileMode,
    ) -> Result<FileDesc, Error>;

    /// Resolve a FD to a path
    fn resolve(&self, fd: FileDesc) -> Result<String, Error>;

    /// Closes an open file descriptor.
    fn close(&self, fd: FileDesc) -> Result<(), Error>;

    /// Reads data from the file descriptor into the provided buffer.
    fn read(&self, fd: FileDesc, buf: &mut [u8]) -> Result<usize, Error>;

    /// Writes data from the given buffer to the file descriptor.
    fn write(&self, fd: FileDesc, buf: &[u8]) -> Result<usize, Error>;

    /// Moves the file offset for the given file descriptor.
    fn lseek(&self, fd: FileDesc, offset: SeekFrom) -> Result<u64, Error>;

    /// Retrieves metadata about the file identified by `fd`.
    fn fstat(&self, fd: FileDesc) -> Result<FileStat, Error>;

    /// Retrieves metadata about the file at `path`.
    fn stat(&self, path: &str) -> Result<FileStat, Error>;

    /// Synchronizes changes in the file referred to by `fd` to storage.
    fn fsync(&self, fd: FileDesc) -> Result<(), Error>;

    /// Removes a file from the file system.
    fn unlink(&self, path: &str) -> Result<(), Error>;

    /// Creates a new directory.
    fn mkdir(&self, path: &str, mode: FileMode) -> Result<(), Error>;

    /// Removes a directory. The directory must be empty.
    fn rmdir(&self, path: &str) -> Result<(), Error>;

    /// Renames a file or directory.
    fn rename(&self, oldpath: &str, newpath: &str) -> Result<(), Error>;

    /// Changes the permissions of a file.
    fn chmod(&self, path: &str, mode: FileMode) -> Result<(), Error>;

    /// Changes the owner and group of a file.
    fn chown(&self, path: &str, owner: u32, group: u32) -> Result<(), Error>;

    /// Reads the contents of a directory, returning a list of directory entries.
    fn readdir(&self, path: &str) -> Result<Vec<String>, Error>;
}
