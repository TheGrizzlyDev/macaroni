const std = @import("std");
const libsystem = @import("./libsystem.zig");
const PathResolver = @import("./PathResolver.zig");

// TODO implement popen and system
pub fn exec(pathResolver: *PathResolver, allocator: *std.mem.Allocator) type {
    return struct {
        var execPathResolver = pathResolver;
        var execAllocator = allocator;

        pub fn execve(path: [*c]const u8, argv: [*c][*c]const u8, envp: [*c][*c]const u8) callconv(.C) c_int {
            const resolved_path = execPathResolver.resolve(execAllocator.*, std.mem.span(path), .{ .sentinel = 0 }) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            // TODO: ensure that the env variables required by macaroni are forwarded
            // TODO: translate libraries search paths too
            return libsystem.execve(@ptrCast(resolved_path), argv, envp);
        }
    };
}
