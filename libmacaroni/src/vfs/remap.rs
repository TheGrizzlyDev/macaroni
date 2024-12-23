use std::{fs::{File, OpenOptions}, os::fd::AsRawFd, path::{Path, PathBuf}, str::FromStr};

use super::vfs::{OpenFlags, VirtualFileSystem};

struct RemapVFS {
    host_path: String,
}

impl RemapVFS {
    fn remap(&self, path: &str) -> Result<PathBuf, super::vfs::Error> {
        let mut resolved_path = PathBuf::from_str(&self.host_path).unwrap();
        let rest = PathBuf::from_str(path).unwrap();
        resolved_path.push(&rest);
        Ok(resolved_path)
    }
}

impl VirtualFileSystem for RemapVFS {
    fn open(
        &self,
        path: &str,
        flags: super::vfs::OpenFlags,
        mode: super::vfs::FileMode,
    ) -> Result<super::vfs::FileDesc, super::vfs::Error> {
        let remapped_path = self.remap(path).unwrap();
        // TODO use OpenOptions
        let f = File::open(remapped_path).unwrap();
        Ok(f.as_raw_fd())
    }

    fn resolve(&self, fd: super::vfs::FileDesc) -> Result<String, super::vfs::Error> {
        todo!()
    }

    fn close(&self, fd: super::vfs::FileDesc) -> Result<(), super::vfs::Error> {
        todo!()
    }

    fn read(&self, fd: super::vfs::FileDesc, buf: &mut [u8]) -> Result<usize, super::vfs::Error> {
        todo!()
    }

    fn write(&self, fd: super::vfs::FileDesc, buf: &[u8]) -> Result<usize, super::vfs::Error> {
        todo!()
    }

    fn lseek(&self, fd: super::vfs::FileDesc, offset: super::vfs::SeekFrom) -> Result<u64, super::vfs::Error> {
        todo!()
    }

    fn fstat(&self, fd: super::vfs::FileDesc) -> Result<super::vfs::FileStat, super::vfs::Error> {
        todo!()
    }

    fn stat(&self, path: &str) -> Result<super::vfs::FileStat, super::vfs::Error> {
        todo!()
    }

    fn fsync(&self, fd: super::vfs::FileDesc) -> Result<(), super::vfs::Error> {
        todo!()
    }

    fn unlink(&self, path: &str) -> Result<(), super::vfs::Error> {
        todo!()
    }

    fn mkdir(&self, path: &str, mode: super::vfs::FileMode) -> Result<(), super::vfs::Error> {
        todo!()
    }

    fn rmdir(&self, path: &str) -> Result<(), super::vfs::Error> {
        todo!()
    }

    fn rename(&self, oldpath: &str, newpath: &str) -> Result<(), super::vfs::Error> {
        todo!()
    }

    fn chmod(&self, path: &str, mode: super::vfs::FileMode) -> Result<(), super::vfs::Error> {
        todo!()
    }

    fn chown(&self, path: &str, owner: u32, group: u32) -> Result<(), super::vfs::Error> {
        todo!()
    }

    fn readdir(&self, path: &str) -> Result<Vec<String>, super::vfs::Error> {
        todo!()
    }
}
