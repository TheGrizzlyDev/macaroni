const std = @import("std");

pub const ResolutionError = error{
    MappingNotFound,
};

pub const Mapping = struct {
    host_path: []const u8,
    sandbox_path: []const u8,
};

pub fn sortMappingByHostPathDesc(_: void, lhs: Mapping, rhs: Mapping) bool {
    return lhs.host_path.len >= rhs.host_path.len;
}

pub fn sortMappingBySandboxPathDesc(_: void, lhs: Mapping, rhs: Mapping) bool {
    return lhs.sandbox_path.len >= rhs.sandbox_path.len;
}

mappings_sorted_by_host_path_desc: []Mapping,
mappings_sorted_by_sandbox_path_desc: []Mapping,
allocator: std.mem.Allocator,

pub fn init(allocator: std.mem.Allocator, mappings: []const Mapping) !@This() {
    // TODO remove trailing '/'
    const mappings_sorted_by_host_path_desc = try allocator.dupe(Mapping, mappings);
    std.mem.sort(Mapping, mappings_sorted_by_host_path_desc, {}, sortMappingByHostPathDesc);
    const mappings_sorted_by_sandbox_path_desc = try allocator.dupe(Mapping, mappings);
    std.mem.sort(Mapping, mappings_sorted_by_sandbox_path_desc, {}, sortMappingBySandboxPathDesc);

    return .{ .allocator = allocator, .mappings_sorted_by_host_path_desc = mappings_sorted_by_host_path_desc, .mappings_sorted_by_sandbox_path_desc = mappings_sorted_by_sandbox_path_desc };
}

pub fn deinit(self: @This()) void {
    self.allocator.free(self.mappings_sorted_by_host_path_desc);
    self.allocator.free(self.mappings_sorted_by_sandbox_path_desc);
}

pub const ResolutionOptions = struct {
    sentinel: ?u8 = null,
};

pub fn resolve(self: @This(), allocator: std.mem.Allocator, path: []const u8, comptime opts: ResolutionOptions) ![]const u8 {
    for (self.mappings_sorted_by_sandbox_path_desc) |mapping| {
        std.debug.print("mapping '{s}' to '{s}'\n", .{ path, mapping.sandbox_path });
        if (mapping.sandbox_path.len > path.len)
            continue;
        if (!std.mem.startsWith(u8, path, mapping.sandbox_path))
            continue;
        const parts = &[_][]const u8{
            mapping.host_path,
            if (std.mem.endsWith(u8, mapping.host_path, "/")) "" else "/",
            path[mapping.sandbox_path.len..],
        };
        return try std.mem.concatMaybeSentinel(allocator, u8, parts, opts.sentinel);
    }
    return ResolutionError.MappingNotFound;
}

pub fn reverse_resolve(self: @This(), allocator: std.mem.Allocator, path: []const u8, comptime opts: ResolutionOptions) ![]const u8 {
    for (self.mappings_sorted_by_host_path_desc) |mapping| {
        if (mapping.host_path.len > path.len)
            continue;
        if (!std.mem.startsWith(u8, path, mapping.host_path))
            continue;
        const parts = &[_][]const u8{
            mapping.sandbox_path,
            if (std.mem.endsWith(u8, mapping.host_path, "/")) "" else "/",
            path[mapping.host_path.len..],
        };
        return try std.mem.concatMaybeSentinel(allocator, u8, parts, opts.sentinel);
    }
    return ResolutionError.MappingNotFound;
}

test "Self::resolve works with a single mapping" {
    const test_resolver = try init(std.testing.allocator, &[_]Mapping{
        .{ .sandbox_path = "/foo", .host_path = "/bar" },
    });
    defer test_resolver.deinit();

    const remapped_path = try test_resolver.resolve(std.testing.allocator, "/foo/file.txt", .{});
    defer std.testing.allocator.free(remapped_path);

    try std.testing.expectEqualStrings("/bar/file.txt", remapped_path);
}

test "Self::resolve uses the longest match when multiple mappings exist" {
    const test_resolver = try init(std.testing.allocator, &[_]Mapping{
        .{ .sandbox_path = "/foo/bar/baz", .host_path = "/expected" },
        .{ .sandbox_path = "/foo/bar", .host_path = "/unexpected" },
    });
    defer test_resolver.deinit();

    const remapped_path = try test_resolver.resolve(std.testing.allocator, "/foo/bar/baz/file.txt", .{});
    defer std.testing.allocator.free(remapped_path);

    try std.testing.expectEqualStrings("/expected/file.txt", remapped_path);
}

test "Self::resolve returns error when no mappings exist" {
    const test_resolver = try init(std.testing.allocator, &[_]Mapping{});
    defer test_resolver.deinit();

    try std.testing.expectError(ResolutionError.MappingNotFound, test_resolver.resolve(std.testing.allocator, "/foo/bar/baz/file.txt", .{}));
}

test "Self::resolve returns error when no mappings matches" {
    const test_resolver = try init(std.testing.allocator, &[_]Mapping{
        .{ .sandbox_path = "/foo", .host_path = "/bar" },
    });
    defer test_resolver.deinit();

    try std.testing.expectError(ResolutionError.MappingNotFound, test_resolver.resolve(std.testing.allocator, "/baz/file.txt", .{}));
}

test "Self::reverse_resolve works with a single mapping" {
    const test_resolver = try init(std.testing.allocator, &[_]Mapping{
        .{ .sandbox_path = "/foo", .host_path = "/bar" },
    });
    defer test_resolver.deinit();

    const remapped_path = try test_resolver.reverse_resolve(std.testing.allocator, "/bar/file.txt", .{});
    defer std.testing.allocator.free(remapped_path);

    try std.testing.expectEqualStrings("/foo/file.txt", remapped_path);
}

test "Self::reverse_resolve uses the longest match when multiple mappings exist" {
    const test_resolver = try init(std.testing.allocator, &[_]Mapping{
        .{ .host_path = "/foo/bar/baz", .sandbox_path = "/expected" },
        .{ .host_path = "/foo/bar", .sandbox_path = "/unexpected" },
    });
    defer test_resolver.deinit();

    const remapped_path = try test_resolver.reverse_resolve(std.testing.allocator, "/foo/bar/baz/file.txt", .{});
    defer std.testing.allocator.free(remapped_path);

    try std.testing.expectEqualStrings("/expected/file.txt", remapped_path);
}

test "Self::reverse_resolve returns error when no mappings exist" {
    const test_resolver = try init(std.testing.allocator, &[_]Mapping{});
    defer test_resolver.deinit();

    try std.testing.expectError(ResolutionError.MappingNotFound, test_resolver.reverse_resolve(std.testing.allocator, "/foo/bar/baz/file.txt", .{}));
}

test "Self::reverse_resolve returns error when no mappings matches" {
    const test_resolver = try init(std.testing.allocator, &[_]Mapping{
        .{ .host_path = "/foo", .sandbox_path = "/bar" },
    });
    defer test_resolver.deinit();

    try std.testing.expectError(ResolutionError.MappingNotFound, test_resolver.reverse_resolve(std.testing.allocator, "/baz/file.txt"));
}
