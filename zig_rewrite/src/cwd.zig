const std = @import("std");
const libsystem = @import("./libsystem.zig");

pub fn getcwd(buf: [*c]u8, size: usize) [*c]u8 {
    std.debug.print("called getcwd\n", .{});
    return libsystem.getcwd(buf, size);
}
