# Installation
Until I get the binaries in package repos for Linux and macOS (Windows in the future), this is the way to go.

## Installer Script (recommended)
A script has been written to get Codeopen installed on your system. Go to the [releases page](https://github.com/ShubhamPatilsd/codeopen/releases/latest) and download `install.sh` or directly download it with [this link](https://github.com/ShubhamPatilsd/codeopen/releases/download/release/install.sh). Then, make it executable by running:
```bash
chmod +x install.sh
```
Then, run `./install.sh` to run the install script!

## Manual Installation
To install it, go to the [release page on GitHub](https://github.com/ShubhamPatilsd/codeopen/releases/). Then, click on the `.zip`/`.tar.xz` file that corresponds to your system (eg: `windows`, `apple`, and `linux`). After extracting the file to your computer, take the executable that pops out and move it to `/usr/local/bin` (or wherever your `$PATH` environment variable points to) if you're on Linux or Mac. 

Here is the command to do that:
```bash
sudo mv codeopen /usr/local/bin/
```


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

You can create as many of these entries as you want in your config file.

And that's it! You can simply just use the tool without exiting your terminal or anything!

# Usage
It's pretty simple to use `codeopen`:
```
codeopen NAME_OF_PROJECT
```
