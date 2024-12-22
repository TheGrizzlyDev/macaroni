#[derive(Debug)]
pub enum LibcResult<T> {
    Ok(T),
    Err(i32),
    ErrAndReturn(T, i32),
    ReturnErr(i32),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InterposeEntry {
    pub replacement: *const (),
    pub original: *const (),
}

unsafe impl Sync for InterposeEntry {}