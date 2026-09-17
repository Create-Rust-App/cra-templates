# Project structure

```text
src/
  main.rs                 # pool + scheduler + shutdown
  lib.rs                  # module root (config, jobs, queue)
  config.rs               # Config::from_env with defaults
  queue.rs                # pool(), Producer, Consumer
  jobs/
    mod.rs                # Job enum + dispatch
    greet.rs              # greet job
    _feature_template.rs  # copy-paste starter for new jobs
tests/
  test_queue.rs           # end-to-end queue tests
```
