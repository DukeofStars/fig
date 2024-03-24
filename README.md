# Fig

A modern, powerful, and truly cross-platform configuration manager.

## FAQ

- **What does "truly cross-platform" mean:**

Many configuration managers technically work on multiple platforms, but they do not allow
for the design differences between them. For example, on Windows, a path might be at %APPDATA%/myapp/config.toml,
but on Linux it might be ~/.config/myapp/config.toml. If the configuration manager only supports paths from $HOME, then
it is
not simple and intuitive to share this configuration between platforms. Fig solves this with namespaces.

- **What is a _namespace_:**

In fig, a namespace is simply a folder, with files/folders in it. It also contains a target, which is a path for it to
be deployed. By **not** sharing this target between systems, you can have groups of configuration files, going to
different folders depending on the system.

## Plugins

When deploying files to your system, Fig can run them through a (or even multiple) program that is on your system.


**NOTE: Fig does not manage or install the plugins for you, they must be already installed and added to the path.**