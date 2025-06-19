# 🧭 Body Weight Tracker (Rust TUI)

A terminal-based body weight tracker with interactive TUI and persistent CSV storage.

---

## Features

- Add daily weight entries via CLI flag or interactive TUI
- Visualize weight history with a line chart in terminal
- Persistent storage in human-readable `weights.csv`
- Prevents duplicate entries per date
- Quit (`q`) and add entry (`a`) keys in TUI mode

---

## Prerequisites

- **Linux only** (tested on Arch Linux)
- Rust toolchain (Rust compiler and Cargo package manager)

---

## Installation Guide

### 1. Clone the repository

```bash
git clone https://github.com/yourusername/body-weight-tracker.git
cd body-weight-tracker
```

```bash
sudo pacman -Syu rust
```

```bash
cargo build --release
```

```bash
cargo install --path .
```

from the root directory of the project — the directory where your `Cargo.toml` file is located.

```bash
weight-tracker --add 70.2
weight-tracker
```
