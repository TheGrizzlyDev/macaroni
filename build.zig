const std = @import("std");

pub fn build(b: *std.Build) void {
    const optimize = b.standardOptimizeOption(.{});

    const target_arm64 = b.resolveTargetQuery(.{
        .cpu_arch = .aarch64,
    });

    const lib_arm64 = b.addSharedLibrary(.{
        .name = "macaroni_arm64",
        .root_source_file = b.path("src/root.zig"),
        .target = target_arm64,
        .optimize = optimize,
    });

    const target_amd64 = b.resolveTargetQuery(.{
        .cpu_arch = .x86_64,
    });

    const lib_amd64 = b.addSharedLibrary(.{
        .name = "macaroni_amd64",
        .root_source_file = b.path("src/root.zig"),
        .target = target_amd64,
        .optimize = optimize,
    });

    const run_lipo = b.addSystemCommand(&.{"lipo"});
    run_lipo.addArgs(&.{ "-create", "-output" });
    const lib = run_lipo.addOutputFileArg("libmacaroni.dylib");
    run_lipo.addArtifactArg(lib_amd64);
    run_lipo.addArtifactArg(lib_arm64);

    b.getInstallStep().dependOn(&b.addInstallLibFile(lib, "libmacaroni.dylib").step);

    const lib_unit_tests = b.addTest(.{
        .root_source_file = b.path("src/root.zig"),
        .optimize = optimize,
    });

    const run_lib_unit_tests = b.addRunArtifact(lib_unit_tests);

    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_lib_unit_tests.step);
}
