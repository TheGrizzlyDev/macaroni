const std = @import("std");

extern "c" fn __error() *c_int;
pub fn setErrno(e: std.posix.E) void {
    const e_ptr = __error();
    e_ptr.* = @intFromEnum(e);
}

pub extern fn getcwd(buf: [*c]u8, size: usize) [*c]u8;
