# Macaroni

This project is similar to [fakeroot](https://wiki.debian.org/FakeRoot) in its approach, but instead of pivoting to a root on the filesystem, it allows you to create a more complex filesystem with mount points pointing at different paths on the underlying hosts. 
This can be useful with buildsystems like Bazel that need to construct potentially large roots for execution build actions, but need to do so with minimal overhead, making even symlinking under the same root prohibitive.

Instead this library intercepts FS related calls from libSystem via DYLD_INSERT_LIBRARIES and the interposition mechanism in mach-o and then remaps the paths in the sandbox to the underlying host paths.

## How to use it

At this point in time, this project is only tested on macos and requires installing the latest zig toolchain. You can then use zig to build the library by running `zig build`. 

Then you can run any process under the sandbox by setting the following env:
- DYLD_INSERT_LIBRARIES: must contain the path to the shared library built by zig
- MACARONI_SANDBOX_ROOT: must be the path to the directory containing the state of the sandbox