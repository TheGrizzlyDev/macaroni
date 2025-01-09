const std = @import("std");
const Mount = @import("./config.zig").Mount;
const utils = @import("./utils.zig");

pub const ResolutionError = error{
    MappingNotFound,
};

pub fn sortMountByHostPathDesc(_: void, lhs: Mount, rhs: Mount) bool {
    return lhs.host_path.len >= rhs.host_path.len;
}

pub fn sortMountBySandboxPathDesc(_: void, lhs: Mount, rhs: Mount) bool {
    return lhs.sandbox_path.len >= rhs.sandbox_path.len;
}

mounts_sorted_by_host_path_desc: []Mount,
mounts_sorted_by_sandbox_path_desc: []Mount,
allocator: std.mem.Allocator,

pub fn init(allocator: std.mem.Allocator, mounts: []const Mount) !@This() {
    // TODO remove trailing '/'
    const mounts_sorted_by_host_path_desc = try allocator.dupe(Mount, mounts);
    std.mem.sort(Mount, mounts_sorted_by_host_path_desc, {}, sortMountByHostPathDesc);
    const mounts_sorted_by_sandbox_path_desc = try allocator.dupe(Mount, mounts);
    std.mem.sort(Mount, mounts_sorted_by_sandbox_path_desc, {}, sortMountBySandboxPathDesc);

    return .{ .allocator = allocator, .mounts_sorted_by_host_path_desc = mounts_sorted_by_host_path_desc, .mounts_sorted_by_sandbox_path_desc = mounts_sorted_by_sandbox_path_desc };
}

pub fn deinit(self: @This()) void {
    self.allocator.free(self.mounts_sorted_by_host_path_desc);
    self.allocator.free(self.mounts_sorted_by_sandbox_path_desc);
}

fn concatPaths(allocator: std.mem.Allocator, first: []const u8, second: []const u8, comptime null_terminated: bool) !MaybeNullTerminatedSlice(u8, null_terminated) {
    const p = try std.fs.path.join(allocator, &[_][]const u8{ first, second });
    if (!null_terminated) {
        return p;
    }
    defer allocator.free(p);
    return allocator.dupeZ(u8, p);
}

fn MaybeNullTerminatedSlice(T: type, null_terminated: bool) type {
    if (null_terminated)
        return [:0]const T;
    return []const T;
}

pub fn resolve(self: @This(), allocator: std.mem.Allocator, path: []const u8, comptime opts: struct { null_terminated: bool = true }) !MaybeNullTerminatedSlice(u8, opts.null_terminated) {
    const realpath = try blk: {
        if (std.mem.startsWith(u8, path, "/"))
            break :blk std.fs.path.resolvePosix(allocator, &[_][]const u8{path});
        const host_cwd = try utils.cwdPath(allocator);
        defer allocator.free(host_cwd);
        const sandbox_cwd = try self.reverse_resolve(allocator, host_cwd, .{});
        defer allocator.free(sandbox_cwd);
        break :blk std.fs.path.resolvePosix(allocator, &[_][]const u8{ sandbox_cwd, path });
    };
    defer allocator.free(realpath);
    for (self.mounts_sorted_by_sandbox_path_desc) |mount| {
        if (mount.sandbox_path.len > realpath.len)
            continue;
        if (!std.mem.startsWith(u8, realpath, mount.sandbox_path))
            continue;
        const resolved_path = try concatPaths(allocator, mount.host_path, realpath[mount.sandbox_path.len..], opts.null_terminated);
        return resolved_path;
    }
    return ResolutionError.MappingNotFound;
}

pub fn reverse_resolve(self: @This(), allocator: std.mem.Allocator, path: []const u8, comptime opts: struct { null_terminated: bool = true }) !MaybeNullTerminatedSlice(u8, opts.null_terminated) {
    const realpath = try std.fs.path.resolvePosix(allocator, &[_][]const u8{path});
    defer allocator.free(realpath);
    for (self.mounts_sorted_by_host_path_desc) |mount| {
        if (mount.host_path.len > realpath.len)
            continue;
        if (!std.mem.startsWith(u8, realpath, mount.host_path))
            continue;
        return concatPaths(allocator, mount.sandbox_path, realpath[mount.host_path.len..], opts.null_terminated);
    }
    return ResolutionError.MappingNotFound;
}

test "Self::resolve works with a single mount" {
    const test_resolver = try init(std.testing.allocator, &[_]Mount{
        .{ .sandbox_path = "/foo", .host_path = "/bar" },
    });
    defer test_resolver.deinit();

    const remapped_path = try test_resolver.resolve(std.testing.allocator, "/foo/file.txt", .{});
    defer std.testing.allocator.free(remapped_path);

    try std.testing.expectEqualStrings("/bar/file.txt", remapped_path);
}

test "Self::resolve uses the longest match when multiple mounts exist" {
    const test_resolver = try init(std.testing.allocator, &[_]Mount{
        .{ .sandbox_path = "/foo/bar/baz", .host_path = "/expected" },
        .{ .sandbox_path = "/foo/bar", .host_path = "/unexpected" },
    });
    defer test_resolver.deinit();

    const remapped_path = try test_resolver.resolve(std.testing.allocator, "/foo/bar/baz/file.txt", .{});
    defer std.testing.allocator.free(remapped_path);

    try std.testing.expectEqualStrings("/expected/file.txt", remapped_path);
}

test "Self::resolve returns error when no mounts exist" {
    const test_resolver = try init(std.testing.allocator, &[_]Mount{});
    defer test_resolver.deinit();

    try std.testing.expectError(ResolutionError.MappingNotFound, test_resolver.resolve(std.testing.allocator, "/foo/bar/baz/file.txt", .{}));
}

test "Self::resolve returns error when no mounts matches" {
    const test_resolver = try init(std.testing.allocator, &[_]Mount{
        .{ .sandbox_path = "/foo", .host_path = "/bar" },
    });
    defer test_resolver.deinit();

    try std.testing.expectError(ResolutionError.MappingNotFound, test_resolver.resolve(std.testing.allocator, "/baz/file.txt", .{}));
}

test "Self::reverse_resolve works with a single mount" {
    const test_resolver = try init(std.testing.allocator, &[_]Mount{
        .{ .sandbox_path = "/foo", .host_path = "/bar" },
    });
    defer test_resolver.deinit();

    const remapped_path = try test_resolver.reverse_resolve(std.testing.allocator, "/bar/file.txt", .{});
    defer std.testing.allocator.free(remapped_path);

    try std.testing.expectEqualStrings("/foo/file.txt", remapped_path);
}

test "Self::reverse_resolve uses the longest match when multiple mounts exist" {
    const test_resolver = try init(std.testing.allocator, &[_]Mount{
        .{ .host_path = "/foo/bar/baz", .sandbox_path = "/expected" },
        .{ .host_path = "/foo/bar", .sandbox_path = "/unexpected" },
    });
    defer test_resolver.deinit();

    const remapped_path = try test_resolver.reverse_resolve(std.testing.allocator, "/foo/bar/baz/file.txt", .{});
    defer std.testing.allocator.free(remapped_path);

    try std.testing.expectEqualStrings("/expected/file.txt", remapped_path);
}

test "Self::reverse_resolve returns error when no mounts exist" {
    const test_resolver = try init(std.testing.allocator, &[_]Mount{});
    defer test_resolver.deinit();

    try std.testing.expectError(ResolutionError.MappingNotFound, test_resolver.reverse_resolve(std.testing.allocator, "/foo/bar/baz/file.txt", .{}));
}

test "Self::reverse_resolve returns error when no mounts matches" {
    const test_resolver = try init(std.testing.allocator, &[_]Mount{
        .{ .host_path = "/foo", .sandbox_path = "/bar" },
    });
    defer test_resolver.deinit();

    try std.testing.expectError(ResolutionError.MappingNotFound, test_resolver.reverse_resolve(std.testing.allocator, "/baz/file.txt", .{}));
}
