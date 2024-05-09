# Fig

A modern, powerful, and truly cross-platform configuration manager.

## Features
 - [x] Truly cross-platform
 - [x] Plugins
 - [x] Namespaces

## FAQ

- **What does "truly cross-platform" mean:**

Many configuration managers technically work on multiple platforms, but they do not allow
for the design differences between them. For example, on Windows, a path might be at %APPDATA%/myapp/config.toml,
but on Linux it might be ~/.config/myapp/config.toml. If the configuration manager only supports paths from $HOME, then
it is not simple and intuitive to share this configuration between platforms. Fig solves this with namespaces.

- **What is a _namespace_:**

In fig, a namespace is simply a folder, with files/folders in it. It also contains a target, which is a path for it to
be deployed. By **not** sharing this target between systems, you can have groups of configuration files, going to
different folders depending on the system.

## Plugins

When deploying files to your system, Fig can run them through a (or even multiple) program that is on your system.
To use a plugin, add it to your plugins.toml file in the fig repository.
`plugins.toml`
```toml
[my-example-plugin]
cmd = "my-example-plugin"
# Trigger on the entire repository, and any file with the extension '.txt'
triggers = ["repo", ".txt"]
```

**NOTE: Fig does not manage or install the plugins for you, they must be already installed and added to the path.**

### How does it work?

The plugin system is designed to be extremely simple, and some cli apps you already use might work out of the box. When running
a plugin on the whole repository, fig simply runs the command, and passes the path to the repository as the first argument.
It also sets the FIG_TRIGGER environment variable to REPOSITORY.
When a plugin is called on a file, it is similar. Fig runs the command, with the path to the file (in the repository, 
not in the system) as the first argument. The FIG_TRGGIER environment variable is set to FILE. The output of this command
is then used to be written to the file in the system. For files, multiple plugins can be called, each with there own extension.
