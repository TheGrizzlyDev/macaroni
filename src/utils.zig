const std = @import("std");

pub fn resolveFd(allocator: std.mem.Allocator, fd: c_int) ![]const u8 {
    var buffer: [std.posix.PATH_MAX]u8 = undefined;
    return allocator.dupe(u8, try std.os.getFdPath(fd, &buffer));
}

pub fn cwdPath(allocator: std.mem.Allocator) ![]const u8 {
    return std.fs.cwd().realpathAlloc(allocator, ".");
}
