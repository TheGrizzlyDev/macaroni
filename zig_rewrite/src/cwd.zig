const std = @import("std");
const libsystem = @import("./libsystem.zig");
const PathResolver = @import("./PathResolver.zig");

pub fn cwd(pathResolver: *PathResolver, allocator: *std.mem.Allocator) type {
    return struct {
        var cwdPathResolver = pathResolver;
        var cwdAllocator = allocator;

        pub fn getcwd(buf: [*c]u8, size: usize) callconv(.C) [*c]u8 {
            const ret_ptr = libsystem.getcwd(buf, size);
            if (ret_ptr == null) {
                return null;
            }

            const resolved_path = cwdPathResolver.reverse_resolve(cwdAllocator.*, std.mem.span(ret_ptr)) catch {
                // std.os.setErrno(std.posix.E.ENOENT);
                return null;
            };
            defer allocator.free(resolved_path);

            if (size == 0) {
                var new_buf = std.heap.c_allocator.alloc(u8, resolved_path.len + 1) catch {
                    // std.os.setErrno(std.posix.E.ENOENT);
                    return null;
                };

                std.mem.copyForwards(u8, new_buf[0..resolved_path.len], resolved_path);
                new_buf[resolved_path.len] = 0;
                return @ptrCast(new_buf);
            }

            if (resolved_path.len >= size) {
                std.debug.print("resolved_path: {s}\n", .{resolved_path});
                // std.os.setErrno(std.posix.E.ERANGE);
                return null;
            }

            std.mem.copyForwards(u8, buf[0..resolved_path.len], resolved_path);
            buf[resolved_path.len] = 0;

            return buf;
        }
    };
}
