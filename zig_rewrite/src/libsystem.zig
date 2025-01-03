const std = @import("std");

extern "c" fn __error() *c_int;
pub fn setErrno(e: std.posix.E) void {
    const e_ptr = __error();
    e_ptr.* = @intFromEnum(e);
}

pub extern fn getcwd(buf: [*c]u8, size: usize) [*c]u8;
pub extern fn open(path: [*c]const u8, oflag: c_int, ...) callconv(.C) c_int;
pub extern fn creat(path: [*c]const u8, mode: std.posix.mode_t) callconv(.C) c_int;
pub extern fn stat(path: [*c]const u8, buf: *anyopaque) callconv(.C) c_int;
pub extern fn chmod(path: [*c]const u8, mode: std.posix.mode_t) callconv(.C) c_int;
pub extern fn chown(path: [*c]const u8, owner: c_int, group: c_int) callconv(.C) c_int;
pub extern fn utimes(path: [*c]const u8, times: *anyopaque) callconv(.C) c_int;
pub extern fn mkdir(path: [*c]const u8, mode: std.posix.mode_t) callconv(.C) c_int;
pub extern fn rmdir(path: [*c]const u8) callconv(.C) c_int;
pub extern fn opendir(path: [*c]const u8) callconv(.C) ?*anyopaque;
pub extern fn execve(path: [*c]const u8, argv: [*c][*c]const u8, envp: [*c][*c]const u8) callconv(.C) c_int;
