// build.zig
const std = @import("std");

fn lazyPath(b: *std.Build, path: []const u8) std.Build.LazyPath {
    return .{ .src_path = .{ .owner = b, .sub_path = path } };
}

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "tokenizer-example",
        .root_source_file = lazyPath(b, "src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Add the library path and link the dylib
    exe.addLibraryPath(lazyPath(b, "./lib"));
    exe.addIncludePath(lazyPath(b, "./src"));
    exe.linkSystemLibrary("tokenizers_sys");
    exe.linkLibC();

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    const run_step = b.step("run", "Run the example");
    run_step.dependOn(&run_cmd.step);
}
