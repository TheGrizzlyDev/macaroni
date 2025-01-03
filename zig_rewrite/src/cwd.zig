const std = @import("std");
const libsystem = @import("./libsystem.zig");
const PathResolver = @import("./PathResolver.zig");

pub fn cwd(pathResolver: *PathResolver, allocator: *std.mem.Allocator) type {
    return struct {
        var cwdPathResolver = pathResolver;
        var cwdAllocator = allocator;

        pub fn getcwd(buf: [*c]u8, size: usize) callconv(.C) [*c]u8 {
            // TODO: this could have arbitrary size as the path prior to mapping
            // is allowed to be shorter than buf
            const ret_ptr = libsystem.getcwd(buf, size);
            if (ret_ptr == null) {
                return null;
            }

            const resolved_path = cwdPathResolver.reverse_resolve(cwdAllocator.*, std.mem.span(ret_ptr), .{
                .sentinel = 0,
            }) catch {
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

            return buf;
        }
    };
}
