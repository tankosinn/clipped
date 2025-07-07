# ✂️ Clipped

Clipped is a workspace-aware, file-scoped `cargo clippy` wrapper that filters diagnostics.

## ✨ Features

- 📁 **File-based filtering**: See only diagnostics for files you specify
- 🚀 **Workspace-aware**: Automatically runs only on relevant workspace packages
- 🎯 **Level filtering**: Control diagnostic severity (note, help, warning, error)
- ⚙️ **Flexible configuration**: CLI, env vars, or config file
- 🔧 **Clippy pass-through**: Forward any arguments to Clippy

## 📦 Installation

```bash
cargo install clipped
```

## 🛠 Usage

```bash
# Run on entire project (same as `cargo clippy`)
clipped

# Run on specific files
clipped src/main.rs src/lib.rs

# Show only errors
clipped --level error src/main.rs

# Pass args to Clippy
clipped src/main.rs -- -- -W clippy::all
```

## ⚙️ Configuration

Configure via `.clipped.toml`:

```toml
level = "error"
clippy_args = ["-W", "clippy::pedantic"]
```

Or set environment variables:

```bash
export CLIPPED_LEVEL=error
export CLIPPED_CLIPPY_ARGS='["--", "-W", "clippy::all"]' # as JSON array
```

## 🤖 CLI

```bash
clipped [OPTIONS] [FILES]... [-- <CLIPPY_ARGS>...]
```

### Options

- `--config <CONFIG_PATH>` - Path to the config file (default: `.clipped.toml`)
- `--level <LEVEL>` - Set the level: `note`, `help`, `warning`, `error` (default: `warning`)
- `-v`, `--verbose` - Enable verbose output
- `-h`, `--help` - Print help
- `-V`, `--version` - Print version

## 🪝 Git Hooks

Clipped is designed to integrate seamlessly with Git hooks.

With `pre-commit`:

```yaml
repos:
  - repo: local
    hooks:
      - id: clipped
        name: clipped
        entry: clipped
        language: system
        args: [...]
        pass_filenames: true
        require_serial: true
```

or use the `clipped` repository:

```yaml
repos:
  - repo: https://github.com/tankosinn/clipped
    rev: 0.1.0
    hooks:
      - id: clipped
```

## 📝 License

[MIT License](https://github.com/tankosinn/clipped/blob/main/LICENSE)
