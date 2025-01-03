const std = @import("std");
const libsystem = @import("./libsystem.zig");
const PathResolver = @import("./PathResolver.zig");

// TODO support *at variants and relative path resolution
pub fn fs(pathResolver: *PathResolver, allocator: *std.mem.Allocator) type {
    return struct {
        var fsPathResolver = pathResolver;
        var fsAllocator = allocator;

        pub fn open(path: [*c]const u8, oflag: c_int, ...) callconv(.C) c_int {
            const remapped_path = fsPathResolver.resolve(allocator.*, std.mem.span(path), .{ .sentinel = 0 }) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            return libsystem.open(@ptrCast(remapped_path), oflag, @cVaStart());
        }

        pub fn creat(path: [*c]const u8, mode: std.posix.mode_t) callconv(.C) c_int {
            const resolved_path = fsPathResolver.resolve(fsAllocator.*, std.mem.span(path), .{ .sentinel = 0 }) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            return libsystem.creat(@ptrCast(resolved_path), mode);
        }

        pub fn stat(path: [*c]const u8, buf: *anyopaque) callconv(.C) c_int {
            const resolved_path = fsPathResolver.resolve(fsAllocator.*, std.mem.span(path), .{ .sentinel = 0 }) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            return libsystem.stat(@ptrCast(resolved_path), buf);
        }

        pub fn chmod(path: [*c]const u8, mode: std.posix.mode_t) callconv(.C) c_int {
            const resolved_path = fsPathResolver.resolve(fsAllocator.*, std.mem.span(path), .{ .sentinel = 0 }) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            return libsystem.chmod(@ptrCast(resolved_path), mode);
        }

        pub fn chown(path: [*c]const u8, owner: c_int, group: c_int) callconv(.C) c_int {
            const resolved_path = fsPathResolver.resolve(fsAllocator.*, std.mem.span(path), .{ .sentinel = 0 }) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            return libsystem.chown(@ptrCast(resolved_path), owner, group);
        }
    };
}
