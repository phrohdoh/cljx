## Purpose
This file gives an AI coding agent the minimal, actionable knowledge to be productive in this repository.

## Big picture
- **Workspace layout:** a small Rust workspace with a top-level binary and several local crates under `crates/`.
- **Primary binary:** built from the root Cargo.toml; results in `target/debug/cljx` and `target/release/cljx`.
- **Core crates:** `crates/cljx_value` (runtime `Value`, `Context`, `Var`), `crates/cljx_float`, `crates/cljx_handle` (handles/interfaces).
- **Binaries & helpers:** shell wrapper `bin/cljx` is the preferred entrypoint for common developer tasks (build, repl, evaluate source).

## Key workflows (commands)
- Build (debug/dev): `$(git root)/bin/cljx build` (dev builds are the default).
- Build release: `$(git root)/bin/cljx build --release`.
- Run REPL: `$(git root)/bin/cljx repl` (preferred; the script forwards to the built binary's `repl` codepath).
- Evaluate source: the `cljx` script/binary can evaluate source from a file via `eval-file file/path.clj` or directly from a source string via `eval-string '(prn :hi)'`.
- Evaluate a sample by name: `$(git root)/bin/cljx eval-sample realish`.
- Use `$(git root)/bin/cljx help` to discover available commands.
- Use `$(git root)/bin/cljx help <command>` for command-specific usage info.

## Project-specific conventions
- The runtime uses shared ownership + interior mutability heavily.
- Values are represented by an enum `Value` in `crates/cljx_value/src/lib.rs`. Bindings use `Var` and `Context::insert/get`.
- Functions are represented via `Function` and the `IFunction`/`RcDynIFunction` abstractions; guest or host-backed callables may be stored in `Handle` objects.

## Files to inspect for examples and authoritative patterns
- Entry & CLI: [crates/cljx_cli/src/main.rs](crates/cljx_cli/src/main.rs)
- Shell wrapper & tasks: [bin/cljx](bin/cljx)
- Runtime model: [crates/cljx/src/value.rs](crates/cljx/src/value.rs)
- Root workspace config: [Cargo.toml](Cargo.toml)
- Usage examples: [README.md](README.md) and [samples/realish.cljx](samples/realish.cljx)

## Integration points & external tooling
- The `bin/cljx` script assumes `pkgx` is available on `PATH`; `pkgx` then provides `yq` and `bb` which the script uses to process embedded YAML tasks.
- Local crates are linked via path deps in `Cargo.toml`.

## Editing guidance for AI agents
- Avoid touching `target/` and other generated build outputs.
- When adding features, follow existing patterns: add types in `crates/*`, expose them via `pub use`, and update `Cargo.toml` path deps.
- For CLI behavior, prefer modifying `src/main.rs` and `bin/cljx` tasks together to keep script and binary in sync.

## Examples to copy/paste
- Start REPL quickly:

```
$(git root)/bin/cljx repl
```

## Notes
- There are no compatibility, stability, security, or performance guarantees of any kind for this workspace.
