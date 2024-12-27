use nix;

#[derive(Debug)]
pub enum LibcResult<T> {
    Ok(T),
    Err(i32),
    ErrAndReturn(T, i32),
    ReturnErr(i32),
}

impl <T> LibcResult<T> {
    pub fn last_error_and_return(value: T) -> LibcResult<T> {
        LibcResult::ErrAndReturn(value, nix::Error::last_raw())
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InterposeEntry {
    pub replacement: *const (),
    pub original: *const (),
}

unsafe impl Sync for InterposeEntry {}