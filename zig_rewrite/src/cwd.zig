const std = @import("std");
const libsystem = @import("./libsystem.zig");
const PathResolver = @import("./PathResolver.zig");

pub fn cwd(pathResolver: *PathResolver) type {
    return struct {
        pub var cwdPathResolver: *PathResolver = pathResolver;

        pub fn getcwd(buf: [*c]u8, size: usize) callconv(.C) [*c]u8 {
            cwdPathResolver.bla();
            std.debug.print("called getcwd\n", .{});
            return libsystem.getcwd(buf, size);
        }
    };
}
