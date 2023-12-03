dotfile manager

I want to learn some Rust and I am ricing my setup, so this feels like a good exercise in CLI development.

# Commands

- `new <package>`: searches for `<package>` in the `$HOME/.config` directory, moves it to `$HOME/.dotfiles` folder and creates a symlink in its place

## TODO
- `list`: lists all packages currently managed by `dot`
- `revert <package>`: opposite of `new`

Any ideas welcome!

