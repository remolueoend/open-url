# open-url

## Description
A small tool for managing custom handlers when opening URLs following a given pattern.

This tool can be used as a default handler of `x-scheme-handler/http(s)` requests, e.g. by creating a custom `.desktop` file and registering it in `~/.config/mimeapps.list`. This tool may come handy when trying to open URLs to files (e.g. on GoogleDrive or other cloud storages) locally whenever possible/synced to local disk.

## Usage
```sh
open-url <url>
```
Run `open-url --help` for more info.

## Configuration
`open-url` expects a config file under `$XDG_CONFIG_HOME/open-url/config.toml` or `XDG_CONFIG_DIRS/open-url/config.toml`. We use the alias `config-dir` for `$XDG_CONFIG_HOME/open-url` or `XDG_CONFIG_DIRS/open-url` in the following paragraphs.
See following block for a valid config example:
```toml
# <config-dir>/config.toml

[[handlers]]
pattern = "https://polybox.ethz.ch/remote.php/webdav/(.*)"
script  = "polybox-file"

[[handlers]]
pattern = "https://polybox.ethz.ch/index.php/apps/files\\?dir=/(.*)"
script  = "polybox-dir"

[[handlers]]
pattern = "https://drive.google.com/(drive/folders|file/d)/(.*)"
script  = "google-drive"
```

Each `handlers` entry consists of a RegEx-pattern for a given URL and the name of the script to run in case of a match. The script is expected to be located at `<config-dir>/scripts/<script-name>` and will be called with following arguments:
1. The URL to open
2. The RegEx pattern used to match the URL

If none of the patterns matched the URL, `open-url` looks for the script `<config-dir>/scripts/_default` and runs it (with an empty pattern as second argument). `_default` should therefore handle all general cases without a custom handler.

Furthermore, depending on the use case, a script may or may not be capable of opening a URL even if its pattern matched the given URL. To solve this, `open-url` reads the exit code of the script and acts accordingly:
* If the script returns successfully (exit code 0), `open-url` assums the script did handle the URL and exits.
* If the script returns a non-zero exit code, `open-url` will instead run the `_default` script.
This allows scripts to reject a URL and forward the request to the default script.


**Important**:
* If `open-url` is registered as default handler for `http(s)` URLs, do not use `xdg-open` or similar commands in any of the scripts, *including* `_default`: This would probably crash your system by repeatingly recursively spawning new processes.
* Make sure all files in `./scripts` are runnable, e.g. by running `chmod +x <script-path>`.

The following code block contains an example script for opening GoogleDrive URLs in your local file explorer / file viewer if the path has been located on your disk (in most cases synced by some external tool):
```bash
#!/usr/bin/env bash

# <config-dir>/scripts/google-drive

# This script resolves the meta data of a GoogleDrive file ID parsed from a GoogleDrive
# URL and tries to lookup the directory or file in the local sync folder.
# When found, the folder/file is opened with the default tool.
# To improve performance, the file meta data are cached locally (needs to be cleaned when files/folders
# are moved on GoogleDrive)
# The URL of a folder/file can be accessed by right-click -> Get Link on GoogleDrive.
# Everything after the folder/file-ID should be removed.
#
# This script requires `gdrive` to be available on your system, which needs to be set up first.

CACHE_DIR="$HOME/.cache/open-url/google-drive"
mkdir -p "$CACHE_DIR"

# match the pattern in $2 against the URL in $1. The pattern contains multiple groups, where as the second
# group contains the folder/file ID (see the pattern in the config example above).
# The group is accessed using BASH_REMATCH:
[[ "$1" =~ $2 ]]
file_id="${BASH_REMATCH[2]}"

if [ -f "$CACHE_DIR/$file_id" ]; then
    echo "found file in cache"
    file_meta=$(cat "$CACHE_DIR/$file_id")
else
    file_meta=$(gdrive info "$file_id")
    echo -e "$file_meta" > "$CACHE_DIR/$file_id"
fi
relative_path=$(echo -e "$file_meta" | grep "Path" | awk -F":" '{print($2)}' | xargs)


local_path="$HOME/GoogleDrive/$relative_path"

if [ -f "$local_path" ] || [ -d "$local_path" ]; then
    echo "$local_path exists"
    # we can safely use xdg-open here as long as url-open is not registered for folder/file URLs:
    xdg-open "$local_path"
    exit 0
else
    echo "$local_path does not exist"
    exit 1
fi
```

## Installation
There exist to distro specific packages (yet). Use the rust stable toolchain to build the binary and copy the output to a folder part of `$PATH`:
```sh
# PWD: root of this repository
cargo build --release
cp ./target/release/open-url <PATH-location>
```

The following samples demonstrate how to register `open-url` as the default handler for `x-scheme/http(s)` requests:
```ini
# ~/.local/share/applications/http.desktop
[Desktop Entry]
Type=Application
Name=Http Browser
Exec=<absolute-path-to-open-url> %U
```

```ini
# ~/.config/mimeapps.list
[Default Applications]
# ...
x-scheme-handler/http=http.desktop
x-scheme-handler/https=http.desktop
# ...
```
Run `update-mime-database` after updating one of the files above.
