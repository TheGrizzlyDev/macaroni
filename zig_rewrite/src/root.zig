const builtin = @import("builtin");
const std = @import("std");
const testing = std.testing;
const libsystem = @import("./libsystem.zig");
const dyld = @import("./dyld.zig");
const cwd = @import("./cwd.zig").cwd(&DEFAULT_PATH_RESOLVER, &GLOBAL_ALLOCATOR);
const PathResolver = @import("./PathResolver.zig");

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
    .{ .original = libsystem.getcwd, .replacement = cwd.getcwd },
};

comptime {
    if (!builtin.is_test) {
        const initPtr = &init;
        @export(initPtr, .{ .name = "init", .linkage = .strong, .section = "__DATA,__mod_init_func" });
    }
}
fn init() callconv(.C) void {
    std.debug.print("init!\n", .{});

    LIBMACARONI_PATH = dyld.findLibraryPath("libmacaroni.dylib") orelse unreachable;

    std.debug.print("Libmacaroni path: {s}\n", .{LIBMACARONI_PATH});

    DEFAULT_PATH_RESOLVER = PathResolver.init(GPA.allocator(), &[_]PathResolver.Mapping{PathResolver.Mapping{ .host_path = "/Users/m1/src/macaroni", .sandbox_path = "/bla" }}) catch unreachable;
}

test {
    _ = @import("./PathResolver.zig");
}
