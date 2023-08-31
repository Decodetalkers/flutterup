# A flutter wrapper


[![Packaging status](https://repology.org/badge/vertical-allrepos/flutterup.svg)](https://repology.org/project/flutterup/versions)

## One easy command to manage and install flutter

Just run *flutterup* to install the flutter SDK or upgrade it

## How does it work

*flutterup* installs symlinks in `/usr/bin` for the following binaries:
- flutter
- dart

linking to `/usr/bin/flutterup` 

When flutter or dart is run, flutterup will install the real target if it is not found, by cloning it from github.

## Why did I create this project

It was difficult for me to package *fluffychat*.

One can clone *flutter* to `~/.cache`, but once in a while, it could be wiped or recloned.

And *flutter* cannot be installed as *root*, because it needs to use a `.git` directory to be updated and to work flawlessly.

And flutter works very much like snap, in that regard, this made me quite unhappy.

## Configuration

It will read its configuration file at `~/.config/flutterup/config.toml`

The syntax is like this

```toml
branch = "stable"
flutter_sdk_dir = "/where/you/want/flutter/to/clone"
````

## TODO

* rustyline cli
	- people could be able to choose the branch to clone to use a different channel
	- Install command could have options, like linux or something else
* better error messages
* meson build script
