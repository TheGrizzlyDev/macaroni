const std = @import("std");

extern fn _dyld_image_count() usize;
extern fn _dyld_get_image_name(image_index: usize) [*c]u8;

pub fn findLibraryPath(name: []const u8) ?[]const u8 {
    const image_count = _dyld_image_count();
    for (0..image_count) |i| {
        const image_name = std.mem.span(_dyld_get_image_name(i));

        if (std.mem.endsWith(u8, image_name, name)) {
            return image_name;
        }
    }
    return null;
}
