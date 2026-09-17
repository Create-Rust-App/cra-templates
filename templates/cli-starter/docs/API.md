# Command reference

## `greet [--name <NAME>] [--shout]`

Print a greeting. Bare invocation greets `CLI_DEFAULT_NAME`.

```sh
cli-starter greet --name Ferris
cli-starter greet --name Ferris --shout
```

## `completions <bash|zsh|fish|powershell>`

Print shell completions to stdout.

```sh
cli-starter completions bash > ~/.local/share/bash-completion/completions/cli-starter
```
