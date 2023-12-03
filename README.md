dotfile manager

I want to learn some Rust and I am (yet again) rewriting all my dotfiles, so this feels like a good exercise in CLI development.

Obligatory works only on my PC.

# Commands

- `new <package>`: searches for `<package>` in the `$HOME/.config` directory, moves it to `$HOME/.dotfiles` folder and creates a symlink in its place

## TODO
- `install`: install all packages in the `$HOME/.dotfiles` directory (--force, optional <package>)
- `list`: lists all packages currently managed by `dot`
- `revert <package>`: opposite of `new`
- configurable dotfiles directory
- add support for non `~/.config/<package>` stuff like `~/.bashrc`

Any ideas welcome!

