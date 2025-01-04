const std = @import("std");
const libsystem = @import("./libsystem.zig");
const PathResolver = @import("./PathResolver.zig");

fn resolveFd(allocator: std.mem.Allocator, fd: c_int) ![]const u8 {
    var buffer: [std.posix.PATH_MAX]u8 = undefined;
    return allocator.dupe(u8, try std.os.getFdPath(fd, &buffer));
}

fn resolveRelative(allocator: std.mem.Allocator, pathResolver: *PathResolver, fd: c_int, path: [*c]const u8) ![]const u8 {
    const parent_host_path = try resolveFd(allocator, fd);
    defer allocator.free(parent_host_path);

    const parent = try pathResolver.reverse_resolve(allocator, parent_host_path, .{});
    defer allocator.free(parent);

    return try std.fs.path.resolve(allocator, &[_][]const u8{ parent, std.mem.span(path) });
}

// TODO support *at variants and relative path resolution
pub fn fs(pathResolver: *PathResolver, allocator: *std.mem.Allocator) type {
    return struct {
        pub fn open(path: [*c]const u8, oflag: c_int, ...) callconv(.C) c_int {
            const remapped_path = pathResolver.resolve(allocator.*, std.mem.span(path), .{ .sentinel = 0 }) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            return libsystem.open(@ptrCast(remapped_path), oflag, @cVaStart());
        }

        pub fn openat(fd: c_int, path: [*c]const u8, oflag: c_int, ...) callconv(.C) c_int {
            const relative_path = resolveRelative(allocator.*, pathResolver, fd, path) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            defer allocator.free(relative_path);

            const remapped_path = pathResolver.resolve(allocator.*, relative_path, .{ .sentinel = 0 }) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            return libsystem.open(@ptrCast(remapped_path), oflag, @cVaStart());
        }

        pub fn creat(path: [*c]const u8, mode: std.posix.mode_t) callconv(.C) c_int {
            const resolved_path = pathResolver.resolve(allocator.*, std.mem.span(path), .{ .sentinel = 0 }) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            return libsystem.creat(@ptrCast(resolved_path), mode);
        }

        pub fn stat(path: [*c]const u8, buf: *anyopaque) callconv(.C) c_int {
            const resolved_path = pathResolver.resolve(allocator.*, std.mem.span(path), .{ .sentinel = 0 }) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            return libsystem.stat(@ptrCast(resolved_path), buf);
        }

        pub fn chmod(path: [*c]const u8, mode: std.posix.mode_t) callconv(.C) c_int {
            const resolved_path = pathResolver.resolve(allocator.*, std.mem.span(path), .{ .sentinel = 0 }) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            return libsystem.chmod(@ptrCast(resolved_path), mode);
        }

        pub fn chown(path: [*c]const u8, owner: c_int, group: c_int) callconv(.C) c_int {
            const resolved_path = pathResolver.resolve(allocator.*, std.mem.span(path), .{ .sentinel = 0 }) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            return libsystem.chown(@ptrCast(resolved_path), owner, group);
        }

        pub fn utimes(path: [*c]const u8, times: *anyopaque) callconv(.C) c_int {
            const resolved_path = pathResolver.resolve(allocator.*, std.mem.span(path), .{ .sentinel = 0 }) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            return libsystem.utimes(@ptrCast(resolved_path), times);
        }

        pub fn mkdir(path: [*c]const u8, mode: std.posix.mode_t) callconv(.C) c_int {
            const resolved_path = pathResolver.resolve(allocator.*, std.mem.span(path), .{ .sentinel = 0 }) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            return libsystem.mkdir(@ptrCast(resolved_path), mode);
        }

        pub fn rmdir(path: [*c]const u8) callconv(.C) c_int {
            const resolved_path = pathResolver.resolve(allocator.*, std.mem.span(path), .{ .sentinel = 0 }) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            return libsystem.rmdir(@ptrCast(resolved_path));
        }

        pub fn opendir(path: [*c]const u8) callconv(.C) ?*anyopaque {
            const resolved_path = pathResolver.resolve(allocator.*, std.mem.span(path), .{ .sentinel = 0 }) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return null;
            };
            return libsystem.opendir(@ptrCast(resolved_path));
        }
    };
}
