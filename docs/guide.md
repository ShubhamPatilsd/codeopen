# Setup

It is simple to create projects that can be opened.

### Config File Location
First, locate your home directory. On Linux, it is `/home/YOUR_USERNAME`, on Mac it is `/Users/YOUR_USERNAME`, and on Windows it's `C:\\Users\YOUR_USERNAME`. The config file should be at `.config/codeopen/config.toml`.

### Config File Setup
If it's not there, create the config file. To set up a project to be opened, follow and fill this TOML structure with your own data and add it to the `config.toml` file:

```java
[[directory_shortcuts]]
name="NAME_OF_PROJECT"
path="PATH_OF_PROJECT"
editor_alias="EDITOR (eg: vim, nano, code)"
```

Just for reference, it would look like `codeopen NAME_OF_PROJECT` on the command line, and if you set `editor_alias` to `code`, it would open it in VSCode. The path is the path to the project. It's preferred if you put it with the `~` character (like `~/programming/project`) but it's okay if you put an absolute file path or a relative file path (the latter being `programming/project`).

And that's it! You can simply just use the tool without exiting your terminal or anything!
