use core::ffi::{c_char, c_int, c_void};
use libc_interposition_macro::interpose;
use libc_interposition_lib::LibcResult;

mod bindings {
    use super::*; 

    extern "C" {
        pub fn posix_spawn(
            pid: *mut c_int,
            path: *const c_char,
            file_actions: *const c_void,
            attrp: *const c_void,
            argv: *const *const c_char,
            envp: *const *const c_char
        ) -> c_int;
    }
}

/// See: man 2 posix_spawn
#[interpose]
pub fn posix_spawn(
    pid: *mut c_int,
    path: *const c_char,
    file_actions: *const c_void, // simplified
    attrp: *const c_void,        // simplified
    argv: *const *const c_char,
    envp: *const *const c_char,
) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 posix_spawnp
#[interpose]
pub fn posix_spawnp(
    pid: *mut c_int,
    file: *const c_char,
    file_actions: *const c_void,
    attrp: *const c_void,
    argv: *const *const c_char,
    envp: *const *const c_char,
) -> LibcResult<c_int> {
    todo!()
}

/// See: man 2 execve
#[interpose]
pub fn execve(
    path: *const c_char,
    argv: *const *const c_char,
    envp: *const *const c_char,
) -> LibcResult<c_int> {
    todo!()
}

/// See: man 3 execv
#[interpose]
pub fn execv(path: *const c_char, argv: *const *const c_char) -> LibcResult<c_int> {
    todo!()
}

/// See: man 3 execvp
#[interpose]
pub fn execvp(file: *const c_char, argv: *const *const c_char) -> LibcResult<c_int> {
    todo!()
}

/// See: man 3 execvpe
#[interpose]
pub fn execvpe(
    file: *const c_char,
    argv: *const *const c_char,
    envp: *const *const c_char,
) -> LibcResult<c_int> {
    todo!()
}

/// See: man 3 execlp
#[interpose]
pub fn execlp(file: *const c_char, arg0: *const c_char, ...) -> LibcResult<c_int> {
    todo!()
}

/// See: man 3 system
#[interpose]
pub fn system(command: *const c_char) -> LibcResult<c_int> {
    todo!()
}

/// See: man 3 popen
#[interpose]
pub fn popen(command: *const c_char, mode: *const c_char) -> LibcResult<*mut c_void> {
    todo!()
}
