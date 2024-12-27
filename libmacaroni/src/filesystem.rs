use core::ffi::{c_char, c_int, c_long, c_size_t, c_uint, c_void};
use libc_interposition_macro::interpose;
use libc_interposition_lib::LibcResult;
use libc;
use crate::path_remapper;

/// See: man 2 open
#[interpose]
pub fn open(path: *const c_char, oflag: c_int, mode: c_int) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path);
    let fd = unsafe { nix::libc::open(remapped_path, oflag, mode) };
    if fd == -1 {
        return LibcResult::last_error_and_return(fd);
    }
    LibcResult::Ok(fd)
}

/// See: man 2 openat  
#[interpose]
pub fn openat(fd: c_int, path: *const c_char, oflag: c_int, mode: c_int) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 creat  
#[interpose]
pub fn creat(path: *const c_char, mode: c_int) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 stat  
#[interpose]
pub fn stat(path: *const c_char, buf: *mut libc::stat) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 lstat  
#[interpose]
pub fn lstat(path: *const c_char, buf: *mut libc::stat) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 fstatat  
#[interpose]
pub fn fstatat(fd: c_int, path: *const c_char, buf: *mut libc::stat, flag: c_int) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 chmod  
#[interpose]
pub fn chmod(path: *const c_char, mode: c_uint) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 lchmod  
#[interpose]
pub fn lchmod(path: *const c_char, mode: c_uint) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 fchmodat  
#[interpose]
pub fn fchmodat(fd: c_int, path: *const c_char, mode: c_uint, flag: c_int) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 chown  
#[interpose]
pub fn chown(path: *const c_char, owner: c_uint, group: c_uint) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 lchown  
#[interpose]
pub fn lchown(path: *const c_char, owner: c_uint, group: c_uint) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 fchownat  
#[interpose]
pub fn fchownat(
    fd: c_int,
    path: *const c_char,
    owner: c_uint,
    group: c_uint,
    flag: c_int,
) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 utimes  
#[interpose]
pub fn utimes(path: *const c_char, times: *const c_void) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 lutimes  
#[interpose]
pub fn lutimes(path: *const c_char, times: *const c_void) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 mkdir  
#[interpose]
pub fn mkdir(path: *const c_char, mode: c_uint) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 mkdirat  
#[interpose]
pub fn mkdirat(fd: c_int, path: *const c_char, mode: c_uint) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 rmdir  
#[interpose]
pub fn rmdir(path: *const c_char) -> LibcResult<c_int> {
    todo!()
}

/// See: man 3 opendir  
#[interpose]
pub fn opendir(name: *const c_char) -> LibcResult<*mut c_void> {
    todo!()
}

/// See: man 2 link  
#[interpose]
pub fn link(oldpath: *const c_char, newpath: *const c_char) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 linkat  
#[interpose]
pub fn linkat(
    olddirfd: c_int,
    oldpath: *const c_char,
    newdirfd: c_int,
    newpath: *const c_char,
    flags: c_int,
) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 unlink  
#[interpose]
pub fn unlink(path: *const c_char) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 unlinkat  
#[interpose]
pub fn unlinkat(dirfd: c_int, path: *const c_char, flags: c_int) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 symlink  
#[interpose]
pub fn symlink(target: *const c_char, linkpath: *const c_char) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 symlinkat  
#[interpose]
pub fn symlinkat(
    target: *const c_char,
    newdirfd: c_int,
    linkpath: *const c_char,
) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 readlink  
#[interpose]
pub fn readlink(path: *const c_char, buf: *mut c_char, bufsize: c_size_t) -> LibcResult<c_long> {
    todo!()
}

/// See: man 2 readlinkat  
#[interpose]
pub fn readlinkat(
    dirfd: c_int,
    path: *const c_char,
    buf: *mut c_char,
    bufsize: c_size_t,
) -> LibcResult<c_long> {
    todo!()
}

/// See: man 2 rename  
#[interpose]
pub fn rename(oldpath: *const c_char, newpath: *const c_char) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 renameat  
#[interpose]
pub fn renameat(
    olddirfd: c_int,
    oldpath: *const c_char,
    newdirfd: c_int,
    newpath: *const c_char,
) -> LibcResult<c_int> {
    todo!()
}

/// See: man 3 remove  
#[interpose]
pub fn remove(path: *const c_char) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 listxattr  
#[interpose]
pub fn listxattr(path: *const c_char, namebuf: *mut c_char, size: c_size_t) -> LibcResult<c_long> {
    todo!()
}

/// See: man 2 llistxattr  
#[interpose]
pub fn llistxattr(path: *const c_char, namebuf: *mut c_char, size: c_size_t) -> LibcResult<c_long> {
    todo!()
}

/// See: man 2 getxattr  
#[interpose]
pub fn getxattr(
    path: *const c_char,
    name: *const c_char,
    value: *mut c_void,
    size: c_size_t,
    position: u32,
    options: c_int,
) -> LibcResult<c_long> {
    todo!()
}

/// See: man 2 lgetxattr  
#[interpose]
pub fn lgetxattr(
    path: *const c_char,
    name: *const c_char,
    value: *mut c_void,
    size: c_size_t,
    position: u32,
    options: c_int,
) -> LibcResult<c_long> {
    todo!()
}

/// See: man 2 setxattr  
#[interpose]
pub fn setxattr(
    path: *const c_char,
    name: *const c_char,
    value: *const c_void,
    size: c_size_t,
    position: u32,
    options: c_int,
) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 lsetxattr  
#[interpose]
pub fn lsetxattr(
    path: *const c_char,
    name: *const c_char,
    value: *const c_void,
    size: c_size_t,
    position: u32,
    options: c_int,
) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 removexattr  
#[interpose]
pub fn removexattr(path: *const c_char, name: *const c_char) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 lremovexattr  
#[interpose]
pub fn lremovexattr(path: *const c_char, name: *const c_char) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 access  
#[interpose]
pub fn access(path: *const c_char, mode: c_int) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 faccessat  
#[interpose]
pub fn faccessat(
    dirfd: c_int,
    path: *const c_char,
    mode: c_int,
    flags: c_int,
) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 chdir  
#[interpose]
pub fn chdir(path: *const c_char) -> LibcResult<c_int> {
    todo!()
}

/// See: man 3 realpath  
#[interpose]
pub fn realpath(path: *const c_char, resolved: *mut c_char) -> LibcResult<*mut c_char> {
    todo!()
}

/// See: man 2 statfs  
#[interpose]
pub fn statfs(path: *const c_char, buf: *mut libc::statfs) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 truncate  
#[interpose]
pub fn truncate(path: *const c_char, length: i64) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 mknod  
#[interpose]
pub fn mknod(path: *const c_char, mode: c_uint, dev: c_uint) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 unmount  
#[interpose]
pub fn unmount(path: *const c_char, flags: c_int) -> LibcResult<c_int> {
    todo!()
}

/// See: man 3 mkfifo  
#[interpose]
pub fn mkfifo(path: *const c_char, mode: c_uint) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 mkfifoat  
#[interpose]
pub fn mkfifoat(fd: c_int, path: *const c_char, mode: c_uint) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 utimensat  
#[interpose]
pub fn utimensat(
    dirfd: c_int,
    path: *const c_char,
    times: *const c_void,
    flags: c_int,
) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 getfh (BSD/macOS)  
#[interpose]
pub fn getfh(path: *const c_char, fhp: *mut c_void) -> LibcResult<c_int> {
    todo!()
}

/// Apple-specific, no standard man page.  
#[interpose]
pub fn open_dprotected_np(
    path: *const c_char,
    flags: c_int,
    protection_class: c_int,
    dpflags: c_int,
    mode: c_int,
) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 searchfs (BSD/macOS)  
#[interpose]
pub fn searchfs(
    path: *const c_char,
    searchblock: *mut c_void,
    resultblock: *mut c_void,
    searchparams: c_uint,
    options: c_int,
) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 fsctl (BSD/macOS)  
#[interpose]
pub fn fsctl(
    path: *const c_char,
    cmd: c_int,
    data: *mut c_void,
    options: c_int,
) -> LibcResult<c_int> {
    todo!()
}
