const builtin = @import("builtin");
const std = @import("std");
const testing = std.testing;
const libsystem = @import("./libsystem.zig");
const dyld = @import("./dyld.zig");
const fs = @import("./fs.zig").fs(&DEFAULT_PATH_RESOLVER, &GLOBAL_ALLOCATOR);
const exec = @import("./exec.zig").exec(&DEFAULT_PATH_RESOLVER, &GLOBAL_ALLOCATOR);
const PathResolver = @import("./PathResolver.zig");
const config = @import("./config.zig");

const Interpose = extern struct { original: *const anyopaque, replacement: *const anyopaque };

var LIBMACARONI_PATH: []const u8 = undefined;
var DEFAULT_PATH_RESOLVER: PathResolver = undefined;
var GPA = std.heap.GeneralPurposeAllocator(.{}){};
var GLOBAL_ALLOCATOR = GPA.allocator();

comptime {
    if (!builtin.is_test) {
        @export(INTERPOSED_SYMBOLS, .{ .name = "INTERPOSED_SYMBOLS", .linkage = .strong, .section = "__DATA,__interpose" });
    }
}
const INTERPOSED_SYMBOLS = [_]Interpose{
    .{ .original = libsystem.getcwd, .replacement = fs.getcwd },
    .{ .original = libsystem.open, .replacement = fs.open },
    .{ .original = libsystem.openat, .replacement = fs.openat },
    .{ .original = libsystem.creat, .replacement = fs.creat },
    .{ .original = libsystem.stat, .replacement = fs.stat },
    .{ .original = libsystem.chmod, .replacement = fs.chmod },
    .{ .original = libsystem.chown, .replacement = fs.chown },
    .{ .original = libsystem.utimes, .replacement = fs.utimes },
    .{ .original = libsystem.mkdir, .replacement = fs.mkdir },
    .{ .original = libsystem.rmdir, .replacement = fs.rmdir },
    .{ .original = libsystem.opendir, .replacement = fs.opendir },
    .{ .original = libsystem.execve, .replacement = exec.execve },
};

comptime {
    if (!builtin.is_test) {
        const initPtr = &init;
        @export(initPtr, .{ .name = "init", .linkage = .strong, .section = "__DATA,__mod_init_func" });
    }
}
fn init() callconv(.C) void {
    initWithError() catch unreachable;
}

fn initWithError() !void {
    LIBMACARONI_PATH = dyld.findLibraryPath("libmacaroni.dylib") orelse unreachable;

    const sandbox_root = try std.process.getEnvVarOwned(GLOBAL_ALLOCATOR, "MACARONI_SANDBOX_ROOT");
    defer GLOBAL_ALLOCATOR.free(sandbox_root);
    const sandbox_config_path = try std.fs.path.resolve(GLOBAL_ALLOCATOR, &[_][]const u8{ sandbox_root, "config.json" });
    defer GLOBAL_ALLOCATOR.free(sandbox_config_path);
    const sandbox_config_file = try std.fs.openFileAbsolute(sandbox_config_path, .{ .mode = .read_only });
    const sandbox_config_content = try sandbox_config_file.readToEndAlloc(GLOBAL_ALLOCATOR, try sandbox_config_file.getEndPos());
    defer GLOBAL_ALLOCATOR.free(sandbox_config_content);
    const config_json = try std.json.parseFromSliceLeaky(config.Config, GLOBAL_ALLOCATOR, sandbox_config_content, .{ .allocate = .alloc_always });
    DEFAULT_PATH_RESOLVER = try PathResolver.init(GPA.allocator(), config_json.mounts);
}

test {
    _ = @import("./PathResolver.zig");
}
