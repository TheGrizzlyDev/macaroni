use core::ffi::{c_char, c_int, c_long, c_size_t, c_uint, c_void};
use std::ffi::CString;
use libc_interposition_macro::interpose;
use libc_interposition_lib::LibcResult;
use crate::path_remapper;

fn remap_at(at_fd: c_int, path: *const c_char) -> Option<CString> {
    use libc::{self, fcntl, F_GETPATH};
    let at_path: *mut c_char = std::ptr::null_mut();
    unsafe {
        _ = fcntl(at_fd, F_GETPATH, at_path); // TODO: handle error
    }
    path_remapper::relative_remap_c_path(at_path, path)
}

/// See: man 2 open
#[interpose]
pub fn open(path: *const c_char, oflag: c_int, mode: c_int) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let fd = unsafe { original(remapped_path.as_ptr(), oflag, mode) };
    if fd == -1 {
        return LibcResult::last_error_and_return(fd);
    }
    LibcResult::return_value(fd)
}

/// See: man 2 openat  
#[interpose]
pub fn openat(fd: c_int, path: *const c_char, oflag: c_int, mode: c_int) -> LibcResult<c_int> {
    let remapped_path = remap_at(fd, path).unwrap();
    let fd = unsafe { libc::open(remapped_path.as_ptr(), oflag, mode) };
    if fd == -1 {
        return LibcResult::last_error_and_return(fd);
    }
    LibcResult::return_value(fd)
}

/// See: man 2 creat  
#[interpose]
pub fn creat(path: *const c_char, mode: c_int) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let fd = unsafe { original(remapped_path.as_ptr(), mode) };
    if fd == -1 {
        return LibcResult::last_error_and_return(fd);
    }
    LibcResult::return_value(fd)
}

/// See: man 2 stat  
#[interpose]
pub fn stat(path: *const c_char, buf: *mut libc::stat) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { original(remapped_path.as_ptr(), buf) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 lstat  
#[interpose]
pub fn lstat(path: *const c_char, buf: *mut libc::stat) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { original(remapped_path.as_ptr(), buf) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 fstatat  
#[interpose]
pub fn fstatat(fd: c_int, path: *const c_char, buf: *mut libc::stat) -> LibcResult<c_int> {
    let remapped_path = remap_at(fd, path).unwrap();
    let ret = unsafe { libc::stat(remapped_path.as_ptr(), buf) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 chmod  
#[interpose]
pub fn chmod(path: *const c_char, mode: c_uint) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { original(remapped_path.as_ptr(), mode) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 lchmod  
#[interpose]
pub fn lchmod(path: *const c_char, mode: c_uint) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { original(remapped_path.as_ptr(), mode) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 fchmodat  
#[interpose]
pub fn fchmodat(fd: c_int, path: *const c_char, mode: c_uint) -> LibcResult<c_int> {
    let remapped_path = remap_at(fd, path).unwrap();
    let ret = unsafe { libc::chmod(remapped_path.as_ptr(), mode.try_into().unwrap()) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 chown  
#[interpose]
pub fn chown(path: *const c_char, owner: c_uint, group: c_uint) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { original(remapped_path.as_ptr(), owner, group) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 lchown  
#[interpose]
pub fn lchown(path: *const c_char, owner: c_uint, group: c_uint) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { nix::libc::chown(remapped_path.as_ptr(), owner, group) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 fchownat  
#[interpose]
pub fn fchownat(
    fd: c_int,
    path: *const c_char,
    owner: c_uint,
    group: c_uint,
) -> LibcResult<c_int> {
    let remapped_path = remap_at(fd, path).unwrap();
    let ret = unsafe { libc::chown(remapped_path.as_ptr(), owner, group) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 utimes  
#[interpose]
pub fn utimes(path: *const c_char, times: *const c_void) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { original(remapped_path.as_ptr(), times) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 lutimes  
#[interpose]
pub fn lutimes(path: *const c_char, times: *const c_void) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { original(remapped_path.as_ptr(), times) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 mkdir  
#[interpose]
pub fn mkdir(path: *const c_char, mode: c_uint) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { original(remapped_path.as_ptr(), mode) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 mkdirat  
#[interpose]
pub fn mkdirat(fd: c_int, path: *const c_char, mode: c_uint) -> LibcResult<c_int> {
    let remapped_path = remap_at(fd, path).unwrap();
    let ret = unsafe { libc::mkdir(remapped_path.as_ptr(), mode.try_into().unwrap()) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 rmdir  
#[interpose]
pub fn rmdir(path: *const c_char) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { nix::libc::rmdir(remapped_path.as_ptr()) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 3 opendir  
#[interpose]
pub fn opendir(path: *const c_char) -> LibcResult<*mut c_void> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let dir = unsafe { nix::libc::opendir(remapped_path.as_ptr()) };
    if dir.is_null() {
        return LibcResult::last_error_and_return(dir.cast());
    }
    LibcResult::return_value(dir.cast())
}

/// See: man 2 link  
#[interpose]
pub fn link(oldpath: *const c_char, newpath: *const c_char) -> LibcResult<c_int> {
    let remapped_oldpath = path_remapper::remap_c_path(oldpath).unwrap();
    let remapped_newpath = path_remapper::remap_c_path(newpath).unwrap();
    let ret = unsafe { original(remapped_oldpath.as_ptr(), remapped_newpath.as_ptr()) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 linkat  
#[interpose]
pub fn linkat(
    olddirfd: c_int,
    oldpath: *const c_char,
    newdirfd: c_int,
    newpath: *const c_char,
) -> LibcResult<c_int> {
    let remapped_oldpath = remap_at(olddirfd, oldpath).unwrap();
    let remapped_newpath = remap_at(newdirfd, newpath).unwrap();
    let ret = unsafe { nix::libc::link(remapped_oldpath.as_ptr(), remapped_newpath.as_ptr()) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 unlink  
#[interpose]
pub fn unlink(path: *const c_char) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { original(remapped_path.as_ptr()) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 unlinkat  
#[interpose]
pub fn unlinkat(dirfd: c_int, path: *const c_char) -> LibcResult<c_int> {
    let remapped_path = remap_at(dirfd, path).unwrap();
    let ret = unsafe { libc::unlink(remapped_path.as_ptr()) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 symlink  
#[interpose]
pub fn symlink(target: *const c_char, linkpath: *const c_char) -> LibcResult<c_int> {
    let remapped_target = path_remapper::remap_c_path(target).unwrap();
    let remapped_link = path_remapper::remap_c_path(linkpath).unwrap();
    let ret = unsafe { original(remapped_target.as_ptr(), remapped_link.as_ptr()) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 3 remove  
#[interpose]
pub fn remove(path: *const c_char) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { original(remapped_path.as_ptr()) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 listxattr  
#[interpose]
pub fn listxattr(path: *const c_char, namebuf: *mut c_char, size: c_size_t) -> LibcResult<c_long> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { original(remapped_path.as_ptr(), namebuf, size) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 access  
#[interpose]
pub fn access(path: *const c_char, mode: c_int) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { nix::libc::access(remapped_path.as_ptr(), mode) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 symlinkat (macOS 10.10+)
#[interpose]
pub fn symlinkat(
    target: *const c_char,
    newdirfd: c_int,
    linkpath: *const c_char,
) -> LibcResult<c_int> {
    let remapped_target = path_remapper::remap_c_path(target).unwrap();
    let remapped_linkpath = remap_at(newdirfd, linkpath).unwrap();

    // Use the "original" symlinkat whenever possible
    let ret = unsafe {
        original(remapped_target.as_ptr(), newdirfd, remapped_linkpath.as_ptr())
    };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 readlink
#[interpose]
pub fn readlink(path: *const c_char, buf: *mut c_char, bufsize: c_size_t) -> LibcResult<c_long> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe {
        // "original" readlink has signature readlink(path, buf, bufsize) -> ssize_t
        original(remapped_path.as_ptr(), buf, bufsize)
    };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 readlinkat (macOS 10.10+)
#[interpose]
pub fn readlinkat(
    dirfd: c_int,
    path: *const c_char,
    buf: *mut c_char,
    bufsize: c_size_t,
) -> LibcResult<c_long> {
    let remapped_path = remap_at(dirfd, path).unwrap();
    let ret = unsafe {
        // "original" readlinkat has signature readlinkat(dirfd, path, buf, bufsize) -> ssize_t
        original(dirfd, remapped_path.as_ptr(), buf, bufsize)
    };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 rename
#[interpose]
pub fn rename(oldpath: *const c_char, newpath: *const c_char) -> LibcResult<c_int> {
    let remapped_old = path_remapper::remap_c_path(oldpath).unwrap();
    let remapped_new = path_remapper::remap_c_path(newpath).unwrap();
    let ret = unsafe {
        // "original" rename has signature rename(old, new)
        original(remapped_old.as_ptr(), remapped_new.as_ptr())
    };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 renameat (macOS 10.10+)
#[interpose]
pub fn renameat(
    olddirfd: c_int,
    oldpath: *const c_char,
    newdirfd: c_int,
    newpath: *const c_char,
) -> LibcResult<c_int> {
    let remapped_old = remap_at(olddirfd, oldpath).unwrap();
    let remapped_new = remap_at(newdirfd, newpath).unwrap();
    let ret = unsafe {
        // "original" renameat has signature renameat(oldfd, old, newfd, new)
        original(
            olddirfd,
            remapped_old.as_ptr(),
            newdirfd,
            remapped_new.as_ptr(),
        )
    };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
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
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    // On macOS, getxattr returns ssize_t
    let ret = unsafe {
        original(
            remapped_path.as_ptr(),
            name,
            value,
            size,
            position,
            options,
        )
    };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
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
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe {
        original(
            remapped_path.as_ptr(),
            name,
            value,
            size,
            position,
            options,
        )
    };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 removexattr
#[interpose]
pub fn removexattr(path: *const c_char, name: *const c_char) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { original(remapped_path.as_ptr(), name) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 faccessat (macOS 10.10+)
#[interpose]
pub fn faccessat(
    dirfd: c_int,
    path: *const c_char,
    mode: c_int,
    flags: c_int,
) -> LibcResult<c_int> {
    let remapped_path = remap_at(dirfd, path).unwrap();
    let ret = unsafe { original(dirfd, remapped_path.as_ptr(), mode, flags) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 chdir
#[interpose]
pub fn chdir(path: *const c_char) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe {
        // "original" chdir(path)
        original(remapped_path.as_ptr())
    };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 3 realpath
#[interpose]
pub fn realpath(path: *const c_char, resolved: *mut c_char) -> LibcResult<*mut c_char> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe {
        // "original" realpath(path, resolved)
        original(remapped_path.as_ptr(), resolved)
    };
    if ret.is_null() {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 statfs
#[interpose]
pub fn statfs(path: *const c_char, buf: *mut libc::statfs) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { original(remapped_path.as_ptr(), buf) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 truncate
#[interpose]
pub fn truncate(path: *const c_char, length: i64) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { original(remapped_path.as_ptr(), length) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 mknod
#[interpose]
pub fn mknod(path: *const c_char, mode: c_uint, dev: c_uint) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    // On macOS, mknod(path, mode_t, dev_t)
    let ret = unsafe {
        original(
            remapped_path.as_ptr(),
            mode,
            dev,
        )
    };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 unmount (macOS)
#[interpose]
pub fn unmount(path: *const c_char, flags: c_int) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    // On macOS, unmount(path, flags)
    let ret = unsafe { original(remapped_path.as_ptr(), flags) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 3 mkfifo (macOS)
#[interpose]
pub fn mkfifo(path: *const c_char, mode: c_uint) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe { original(remapped_path.as_ptr(), mode) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 mkfifoat (macOS 10.10+)
#[interpose]
pub fn mkfifoat(fd: c_int, path: *const c_char, mode: c_uint) -> LibcResult<c_int> {
    let remapped_path = remap_at(fd, path).unwrap();
    // "original" mkfifoat(dirfd, path, mode_t)
    let ret = unsafe { original(fd, remapped_path.as_ptr(), mode) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 utimensat (macOS 10.13+)
#[interpose]
pub fn utimensat(
    dirfd: c_int,
    path: *const c_char,
    times: *const c_void,
    flags: c_int,
) -> LibcResult<c_int> {
    let remapped_path = remap_at(dirfd, path).unwrap();
    // "original" utimensat(dirfd, pathname, times, flags)
    let ret = unsafe { original(dirfd, remapped_path.as_ptr(), times, flags) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 getfh (BSD/macOS)
#[interpose]
pub fn getfh(path: *const c_char, fhp: *mut c_void) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    // "original" getfh(path, fhandle_t*)
    let ret = unsafe { original(remapped_path.as_ptr(), fhp) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// Apple-specific, no standard man page: open_dprotected_np
#[interpose]
pub fn open_dprotected_np(
    path: *const c_char,
    flags: c_int,
    protection_class: c_int,
    dpflags: c_int,
    mode: c_int,
) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    let ret = unsafe {
        original(
            remapped_path.as_ptr(),
            flags,
            protection_class,
            dpflags,
            mode,
        )
    };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 searchfs (macOS)
#[interpose]
pub fn searchfs(
    path: *const c_char,
    searchblock: *mut c_void,
    resultblock: *mut c_void,
    searchparams: c_uint,
    options: c_int,
) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    // "original" searchfs(path, searchblock, resultblock, searchparams, options)
    let ret = unsafe { original(remapped_path.as_ptr(), searchblock, resultblock, searchparams, options) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}

/// See: man 2 fsctl (macOS)
#[interpose]
pub fn fsctl(
    path: *const c_char,
    cmd: c_int,
    data: *mut c_void,
    options: c_int,
) -> LibcResult<c_int> {
    let remapped_path = path_remapper::remap_c_path(path).unwrap();
    // "original" fsctl(path, cmd, data, options)
    let ret = unsafe { original(remapped_path.as_ptr(), cmd, data, options) };
    if ret == -1 {
        return LibcResult::last_error_and_return(ret);
    }
    LibcResult::return_value(ret)
}
