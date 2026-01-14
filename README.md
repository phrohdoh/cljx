
### tests

```shell
cargo watch -s "$(git root)/bin/cljx test-crates --all"
```

```shell
cargo watch -s "$(git root)/bin/cljx test-crates read -- multi-line"
```

### bin files

#### `cljx`

```shell
$(git root)/bin/cljx --help
```

#### `cljx` repl

```shell
$(git root)/bin/cljx repl
```

#### `cljx` eval file

```shell
$(git root)/bin/cljx eval-file $(git root)/samples/old.cljx
```

```shell
$(git root)/bin/cljx eval-file <(printf '(prn :hi)')
```
