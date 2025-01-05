const std = @import("std");
const libsystem = @import("./libsystem.zig");
const PathResolver = @import("./PathResolver.zig");
const utils = @import("./utils.zig");

fn resolveRelative(allocator: std.mem.Allocator, path_resolver: *PathResolver, fd: c_int, path: [*c]const u8) ![]const u8 {
    const parent_host_path = try utils.resolveFd(allocator, fd);
    defer allocator.free(parent_host_path);

    const parent = try path_resolver.reverse_resolve(allocator, parent_host_path, .{ .null_terminated = false });
    defer allocator.free(parent);

    return try std.fs.path.resolve(allocator, &[_][]const u8{ parent, std.mem.span(path) });
}

// TODO support *at variants and relative path resolution
pub fn fs(path_resolver: *PathResolver, allocator: *std.mem.Allocator) type {
    return struct {
        pub fn open(path: [*c]const u8, oflag: c_int, ...) callconv(.C) c_int {
            const remapped_path = path_resolver.resolve(allocator.*, std.mem.span(path), .{}) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            defer allocator.free(remapped_path);
            var va = @cVaStart();
            defer @cVaEnd(&va);
            return libsystem.open(@ptrCast(remapped_path), oflag, @cVaCopy(&va));
        }

        pub fn openat(fd: c_int, path: [*c]const u8, oflag: c_int, ...) callconv(.C) c_int {
            const relative_path = resolveRelative(allocator.*, path_resolver, fd, path) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            defer allocator.free(relative_path);

            const remapped_path = path_resolver.resolve(allocator.*, relative_path, .{}) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            defer allocator.free(remapped_path);

            var va = @cVaStart();
            defer @cVaEnd(&va);
            return libsystem.open(@ptrCast(remapped_path), oflag, @cVaCopy(&va));
        }

        pub fn creat(path: [*c]const u8, mode: std.posix.mode_t) callconv(.C) c_int {
            const resolved_path = path_resolver.resolve(allocator.*, std.mem.span(path), .{}) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            defer allocator.free(resolved_path);
            return libsystem.creat(@ptrCast(resolved_path), mode);
        }

        pub fn stat(path: [*c]const u8, buf: *anyopaque) callconv(.C) c_int {
            const resolved_path = path_resolver.resolve(allocator.*, std.mem.span(path), .{}) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            defer allocator.free(resolved_path);
            return libsystem.stat(@ptrCast(resolved_path), buf);
        }

        pub fn chmod(path: [*c]const u8, mode: std.posix.mode_t) callconv(.C) c_int {
            const resolved_path = path_resolver.resolve(allocator.*, std.mem.span(path), .{}) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            defer allocator.free(resolved_path);
            return libsystem.chmod(@ptrCast(resolved_path), mode);
        }

        pub fn chown(path: [*c]const u8, owner: c_int, group: c_int) callconv(.C) c_int {
            const resolved_path = path_resolver.resolve(allocator.*, std.mem.span(path), .{}) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            defer allocator.free(resolved_path);
            return libsystem.chown(@ptrCast(resolved_path), owner, group);
        }

        pub fn utimes(path: [*c]const u8, times: *anyopaque) callconv(.C) c_int {
            const resolved_path = path_resolver.resolve(allocator.*, std.mem.span(path), .{}) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            defer allocator.free(resolved_path);
            return libsystem.utimes(@ptrCast(resolved_path), times);
        }

        pub fn mkdir(path: [*c]const u8, mode: std.posix.mode_t) callconv(.C) c_int {
            const resolved_path = path_resolver.resolve(allocator.*, std.mem.span(path), .{}) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            defer allocator.free(resolved_path);
            return libsystem.mkdir(@ptrCast(resolved_path), mode);
        }

        pub fn rmdir(path: [*c]const u8) callconv(.C) c_int {
            const resolved_path = path_resolver.resolve(allocator.*, std.mem.span(path), .{}) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return -1;
            };
            defer allocator.free(resolved_path);
            return libsystem.rmdir(@ptrCast(resolved_path));
        }

        pub fn opendir(path: [*c]const u8) callconv(.C) ?*anyopaque {
            const resolved_path = path_resolver.resolve(allocator.*, std.mem.span(path), .{}) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return null;
            };
            defer allocator.free(resolved_path);
            return libsystem.opendir(@ptrCast(resolved_path));
        }

        pub fn getcwd(buf: [*c]u8, size: usize) callconv(.C) [*c]u8 {
            const host_cwd = utils.cwdPath(allocator.*) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return null;
            };
            defer allocator.free(host_cwd);
            const resolved_path = path_resolver.reverse_resolve(allocator.*, host_cwd, .{}) catch {
                libsystem.setErrno(std.posix.E.NOENT);
                return null;
            };
            defer allocator.free(resolved_path);

            if (size == 0) {
                const new_buf = std.heap.c_allocator.dupeZ(u8, resolved_path) catch {
                    libsystem.setErrno(std.posix.E.NOENT);
                    return null;
                };

                return @ptrCast(new_buf);
            }

            if (resolved_path.len >= size) {
                libsystem.setErrno(std.posix.E.RANGE);
                return null;
            }

            std.mem.copyForwards(u8, buf[0..resolved_path.len], resolved_path);
            buf[resolved_path.len] = 0;

            return buf;
        }
    };
}
