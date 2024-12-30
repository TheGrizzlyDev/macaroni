const std = @import("std");

str: []const u8,

pub fn bla(self: @This()) void {
    std.debug.print("bla {s}\n", .{self.str});
}
