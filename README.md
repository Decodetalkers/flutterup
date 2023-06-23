# A flutter wrapper


[![Packaging status](https://repology.org/badge/vertical-allrepos/flutterup.svg)](https://repology.org/project/flutterup/versions)

## Just run flutterup , and install the flutter

## How it works

I think after it will create soft link under /usr/bin

* flutterup
* flutter
* dart

If program can notfind the real target, it will clone the real one from github

## Why create

It is difficult for me to package fluffychat, Some one will clone it to ~/.cache, but every time, it will be clean or reclone. And flutter cannot just put to root, because it need `.git` to update and work. And flutter like snap very much, this make me unhappy

## Configure

It will read a config in `~/.config/flutterup/config.toml`

configure is like

```toml
branch = "stable"
flutter_sdk_dir = "/where/you/want/flutter/to/clone"
````

## TODO

* rustyline cli
	I think people can choose branch to clone to use default channel
	Install needed option, like linux or something else
* better error
* meson build script
