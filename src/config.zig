pub const Mount = struct {
    host_path: []const u8,
    sandbox_path: []const u8,
};

pub const Config = struct {
    mounts: []const Mount,
};
