# Project structure

```text
src/
  main.rs                 # tracing setup + CLI dispatch
  lib.rs                  # module root (cli, commands, config)
  cli.rs                  # clap Cli + Commands enum
  config.rs               # Config::from_env with defaults
  commands/
    mod.rs                # run() dispatch
    greet.rs              # greet subcommand
    completions.rs        # completions subcommand
    _feature_template.rs  # copy-paste starter for new subcommands
tests/
  test_cli.rs             # end-to-end CLI surface tests
```
