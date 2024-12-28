use nix::{Error};

#[derive(Debug)]
pub enum PropagateErrno {
    Last,
    Override(Error),
}

#[derive(Debug)]
pub struct LibcResult<T> {
    pub err: Option<PropagateErrno>,
    pub val:   T,
}

impl <T> LibcResult<T> {
    pub fn return_value(val: T) -> LibcResult<T> {
        Self { err: None, val }
    }

    pub fn last_error_and_return(val: T) -> LibcResult<T> {
        Self { err: Some(PropagateErrno::Last), val }
    }

    pub fn override_error_and_return(err: Error, val: T) -> LibcResult<T> {
        Self { err: Some(PropagateErrno::Override(err)), val }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InterposeEntry {
    pub replacement: *const (),
    pub original: *const (),
}

unsafe impl Sync for InterposeEntry {}