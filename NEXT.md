1. transition back to a sandbox directory with a config.json
2. add paths to stage symlinks to SIP protected binaries
3. add PATH resolution in exec calls to check if the binary about to be invoked is SIP protected
4. build environment based on lib path and sandbox path and implement exec calls
5. write path resolver and hook it to each FS API
6. correctly follow symlinks
7. implement *-at functions
8. add DYLD paths that follow the sandbox