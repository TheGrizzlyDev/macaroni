const std = @import("std");
const testing = std.testing;

const libsystem = @import("./libsystem.zig");
const cwd = @import("./cwd.zig");

const Interpose = extern struct { original: *const anyopaque, replacement: *const anyopaque };

comptime {
    @export(INTERPOSED_SYMBOLS, .{ .name = "INTERPOSED_SYMBOLS", .linkage = .strong, .section = "__DATA,__interpose" });
}
const INTERPOSED_SYMBOLS = [_]Interpose{.{ .original = libsystem.getcwd, .replacement = cwd.getcwd }};
