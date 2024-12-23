pub trait GenericVirtualFileSystem {
    type FileDesc;
    type Error;
    type FileStat;
    type SeekFrom;
    type FileMode;
    type OpenFlags;

    /// Opens a file at the given `path` with the specified flags and mode.
    fn open(
        &self,
        path: &str,
        flags: Self::OpenFlags,
        mode: Self::FileMode,
    ) -> Result<Self::FileDesc, Self::Error>;

    /// Resolve a FD to a path
    fn resolve(&self, fd: Self::FileDesc) -> Result<String, Self::Error>;

    /// Closes an open file descriptor.
    fn close(&self, fd: Self::FileDesc) -> Result<(), Self::Error>;

    /// Reads data from the file descriptor into the provided buffer.
    fn read(&self, fd: Self::FileDesc, buf: &mut [u8]) -> Result<usize, Self::Error>;

    /// Writes data from the given buffer to the file descriptor.
    fn write(&self, fd: Self::FileDesc, buf: &[u8]) -> Result<usize, Self::Error>;

    /// Moves the file offset for the given file descriptor.
    fn lseek(&self, fd: Self::FileDesc, offset: Self::SeekFrom) -> Result<u64, Self::Error>;

    /// Retrieves metadata about the file identified by `fd`.
    fn fstat(&self, fd: Self::FileDesc) -> Result<Self::FileStat, Self::Error>;

    /// Retrieves metadata about the file at `path`.
    fn stat(&self, path: &str) -> Result<Self::FileStat, Self::Error>;

    /// Synchronizes changes in the file referred to by `fd` to storage.
    fn fsync(&self, fd: Self::FileDesc) -> Result<(), Self::Error>;

    /// Removes a file from the file system.
    fn unlink(&self, path: &str) -> Result<(), Self::Error>;

    /// Creates a new directory.
    fn mkdir(&self, path: &str, mode: Self::FileMode) -> Result<(), Self::Error>;

    /// Removes a directory. The directory must be empty.
    fn rmdir(&self, path: &str) -> Result<(), Self::Error>;

    /// Renames a file or directory.
    fn rename(&self, oldpath: &str, newpath: &str) -> Result<(), Self::Error>;

    /// Changes the permissions of a file.
    fn chmod(&self, path: &str, mode: Self::FileMode) -> Result<(), Self::Error>;

    /// Changes the owner and group of a file.
    fn chown(&self, path: &str, owner: u32, group: u32) -> Result<(), Self::Error>;

    /// Reads the contents of a directory, returning a list of directory entries.
    fn readdir(&self, path: &str) -> Result<Vec<String>, Self::Error>;
}

mod macos {
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

    pub type FileDesc = u32;
    pub enum Error {}
}

#[cfg(target_os = "macos")]
pub trait VirtualFileSystem =
    GenericVirtualFileSystem<
    FileDesc = macos::FileDesc,
    Error = macos::Error,
    FileStat = macos::FileStat,
    SeekFrom = macos::SeekFrom,
    FileMode = macos::FileMode,
    OpenFlags = macos::OpenFlags,
>;