# NOTE #

Unfortunately, I've decided to axe this project. While there were features in a dotfile manager that I would have liked and have listed in this project's spec, there are other completed and wildly used managers that can be made to work the way I would like. Next time, I'll take a better look at what's available before I start a new project. On the positive side, I learned quite a lot about rust while working on this!

# dotty #
[![Build Status](https://travis-ci.org/Lindenk/dotty.svg?branch=master)](https://travis-ci.org/Lindenk/dotty)
[![Build status](https://ci.appveyor.com/api/projects/status/ob7ueebe01pgt4vb/branch/master?svg=true)](https://ci.appveyor.com/project/Lindenk/dotty/branch/master)
[![Stories in Ready](https://badge.waffle.io/Lindenk/dotty.svg?label=ready&title=Ready)](http://waffle.io/Lindenk/dotty)


Dotty is an advanced dotfile manager based on GNU stow with a focus on keep configurations seperate on a per-program basis. It supports features such as:

- Dependancies
- Install/remove hooks
- Dotfile generation
- Configuation with YAML or JSON

### Future Features ##

- Automatically updates/syncs, can be configured. (This would require dotty to spawn a daemon)
- GUI and/or ncurses interface.
- Support for multiple repositories / external modules.

## Installation ##

Currently there aren't any built binaries to download (although that might come in the future). Dotty can be built using the nightly rust like so:

```
cargo build --release
```

## Quick Start ##

First, make a directory containing the config files you want to version:

```
- dotfiles
    - zsh
        - .zshenv
        - .zprofile
        - .zshrc
```

Then, add a `module.yml` file for dotty to read:

```
- dotfiles 
    - zsh 
        - module.yml
        - .zshenv 
        - .zprofile 
        - .zshrc
```

In this example, we will use the following `module.yml` configuration:

```yaml 
name: zsh 
links: 
    - .zshenv:~/.zshenv 
    - .zprofile:~/.zprofile 
    - .zshrc:~/.zshrc 
```

Now install your new module as `zsh`:

```
dotty install dotfiles/zsh 
```

To remove it:

```
dotty remove zsh 
```

## Reference ##

With dotty, you can:
- Install a module
- Remove a module
- Update an installed module
- Reinstall an install module
- Query dotty for information

### Install ###
Installs a module or modules to the target directory.

```bash
dotty install [MODULE_NAME]...
```

When installing a module, the following will occur in order:

- The module's dependancies will be resolved and installed in reverse order
- Any installation hooks for the module will run
- All files will be symlinked or generated to their respective locaitons
- Any post installation hooks for the module will run

The following flags are supported for `dotty install`:

```
(Not implemented yet)
-f      Force install all files, even if they overwrite exiting ones
```



### Remove ###
Remove files associated with a given module(s) and uninstalls the module(s).

```bash
dotty remove [MODULE_NAME]...
```

When removing an installed module, the following will occur in order:

- Any removal hooks will be run for the module
- All symlinks and generated files created by `install` will be deleted
- Any post removal hooks will be run for the module

The following flags are supported for `dotty remove`:

```
(Not implemented yet)
-r      Recursively remove all dependancies installed by this module if they are not being used
        by another module and were not explicitly installed
```

### Reinstall ###
(Not implemented yet)

Effectively runs `remove` on a module, then runs `install` on it.

```bash
dotty reinstall [MODULE_NAME]...
```

The following flags are support for `dotty reinstall`:

```
-r      Recursively reinstall all dependancies as well
-f      Force install any new files that already exist in the filesystem
```

### Update ###
(Not implemented yet)

Recompiles any generated files and re-symlinks all files. Does not run hooks.

```bash
dotty update [MODULE_NAME]...
```

The following flags are support for `dotty update`:

```
-r      Recursively update all dependancies as well
-f      Force install any new files that already exist in the filesystem
```

If no module is specified, all installed modules will be updated.

### List ###
Lists modules. Will print all installed modules by default.

```bash
dotty list installed
dotty list all
dotty list files [MODULE_NAME]
```


## Module Configuration ##

Modules are configured using YAML (and by extension JSON). Example:

Assume a module filesystem of:

```
module.yml
config
    .Xresources
    .config
        prog
            stuff.conf
```

```yaml
links:
    - config:~/.config

(Not implemented yet)
append:
    .Xresources: config/.Xresources
hooks:
    OnInstall: ["script.sh", "prog"]
    OnRemove: "rScript.sh"
    OnPostInstall: []
    OnPostRemove: 
        - cleanup.sh
        - cleanupother.sh
dependancies:
    - othermodule
```

## Dotty Configuration ##

Dotty itself can be configured with a config file in `~/.config/dotty/config.yml` by default. The following configurations are supported:

```yaml
default_data_dir: "/path/to/data/dir"
```
