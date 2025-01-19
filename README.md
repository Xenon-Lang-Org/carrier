# Carrier CLI

This is a CLI tool for the Xenon language, which includes requires the following components to be used effectively:

- A compiler (`xcc`) that compiles `.xn` source files into `.wasm`.
- An interpreter (`xin`) that can run `.xn` code directly.
- A WASM virtual machine (`xrun`) that executes compiled `.wasm` files.

`Carrier` CLI wraps these tools and manages project creation, configuration, building, and running code.

---

## Installation

1. Clone or download this repository.
2. In the root of the project, run:
   ```bash
   cargo install --path .
   ```
   This installs the `carrier` binary into your Cargo bin directory.

_Alternatively, for local testing without installing globally, just run:_
```bash
cargo run -- <command> [args...]
```

---

## Commands & Usage

### 1. `carrier init <project-name>`

Initializes a new xenon project in the specified directory:
- Creates the directory (e.g. `myproject/`).
- Creates a `src/` folder containing a basic `main.xn`.
- Creates a `carrier.toml` configuration file with default settings.

**Example**:
```bash
carrier init myproject
```
Directory structure after init:
```
myproject
├── carrier.toml
└── src
    └── main.xn
```

---

### 2. `carrier build`

Builds (compiles) your Xenon source files into a single `.wasm` module. By default:
- Gathers **all** `.xn` files from the `src/` directory (recursively).
- Concatenates them into `out/output.xn`.
- Invokes the compiler (`xcc`) to produce `out/output.wasm`.

You may override:
- The source file (using `-s`/`--source`).
- The output path (using `-o`/`--output`).

**Examples**:

1. **Default build**:
   ```bash
   xn build
   ```
   This will produce:
   - `out/output.xn` (concatenated source)
   - `out/output.wasm` (compiled binary)

2. **Build from a single file**:
   ```bash
   xn build --source main.xn --output myapp.wasm
   ```
   This skips gathering files in `src/`, instead using `main.xn` directly.

---

### 3. `carrier run`

Runs (interprets) `.xn` files using `xin`. By default:
- Looks for **all** `.xn` files in `src/` if no arguments are given.

**Examples**:

1. **Run all `.xn` in `src/`**:
   ```bash
   xn run
   ```
   
2. **Run specific files**:
   ```bash
   xn run file1.xn file2.xn
   ```
   This ignores any `.xn` files that aren’t listed.

3. **Pass an entry file**:
   ```bash
   xn run file1.xn file2.xn --entry main.xn
   ```
   The interpreter will receive `-e main.xn`.

---

### 4. `carrier vm <wasm-file> [args...]`

Executes a compiled `.wasm` file using the configured WASM VM (default: `xrun`).

**Example**:
```bash
carrier vm out/output.wasm arg1 arg2
```
Which will effectively run:
```bash
xrun out/output.wasm arg1 arg2
```

---

### 5. `carrier config [<key> [<value>]]`

View or modify project configuration stored in `carrier.toml`.

- **No arguments**: Prints all current config values.
- **One argument** (`<key>`): Prints the value of that key.
- **Two arguments** (`<key> <value>`): Sets the key to the new value, then updates `carrier.toml`.

**Examples**:

1. **List all config**:
   ```bash
   carrier config
   ```
2. **Get a single config key**:
   ```bash
   carrier config compiler_path
   ```
3. **Set a config key**:
   ```bash
   carrier config compiler_path /usr/local/bin/xcc
   ```

---

## Typical Workflow

1. **Initialize a new project**:
   ```bash
   carrier init myproject
   cd myproject
   ```
2. **Build** all `.xn` files in `src/`:
   ```bash
   carrier build
   ```
   Outputs `out/output.wasm`.
3. **Run** the code with the interpreter:
   ```bash
   carrier run
   ```
4. **Execute** the compiled WASM in the VM:
   ```bash
   carrier vm out/output.wasm
   ```

---

## Customization

The CLI reads and writes configuration to the local `carrier.toml`. Fields like `compiler_path`, `interpreter_path`, and `vm_path` can be changed so the CLI points to different binaries or scripts as needed.  

Use `carrier config` to quickly view or modify these paths.

---

## Directory Structure

A typical project layout might look like:

```
myproject
├── carrier.toml             # Project configuration
└── src
    ├── main.xn         # Entry point
    ├── lib.xn          # Additional code
    └── submodule
        └── sub.xn
```

When running `carrier build`, all `.xn` files in `src/` and subdirectories are concatenated into `out/output.xn` and then compiled to `out/output.wasm`.
