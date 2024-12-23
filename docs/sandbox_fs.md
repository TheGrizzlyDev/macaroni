# Sandbox FS

Sandbox FS is the structure of each individual sandbox. It is designed so that each process can use it without an external daemon to coordinate them, which helps with performances.

/ <- root of the FS
/config.json <- describes the shared configuration of all processes in a sandbox
/pid/{real-pid}
    / ...

# Config file

```
{
    "mounts": [
        {
            "type": "remap",
            "destination_path": "/",
            "host_path": "/some/path/",
            ... maybe other attributes ...
        }
    ]
}
```