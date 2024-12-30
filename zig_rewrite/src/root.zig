const std = @import("std");
const testing = std.testing;
const libsystem = @import("./libsystem.zig");
const cwd = @import("./cwd.zig").cwd(&DEFAULT_PATH_RESOLVER);
const PathResolver = @import("./PathResolver.zig");

const Interpose = extern struct { original: *const anyopaque, replacement: *const anyopaque };

var DEFAULT_PATH_RESOLVER: PathResolver = undefined;

comptime {
    @export(INTERPOSED_SYMBOLS, .{ .name = "INTERPOSED_SYMBOLS", .linkage = .strong, .section = "__DATA,__interpose" });
}
const INTERPOSED_SYMBOLS = [_]Interpose{
    .{ .original = libsystem.getcwd, .replacement = cwd.getcwd },
};

comptime {
    const initPtr = &init;
    @export(initPtr, .{ .name = "init", .linkage = .strong, .section = "__DATA,__mod_init_func" });
}
fn init() callconv(.C) void {
    std.debug.print("init!\n", .{});
    DEFAULT_PATH_RESOLVER.str = "bla";
}

test {
    _ = @import("./PathResolver.zig");
}
