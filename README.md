# runitgui

A terminal-based (TUI) application for managing runit services.

## Overview

runitgui provides a user-friendly interface to view and manage runit supervised services. It displays all available services, their status, and allows you to start, stop, restart, enable, and disable them.

## Requirements

- Rust 1.70 or later
- runit installed on the system
- Root privileges for full functionality

## Installation

```bash
cargo build --release
```

## Usage

Run with root privileges for full functionality:

```bash
sudo ./target/release/runitgui
```

Or from the project directory:

```bash
cd runitgui
cargo run --release
```

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `↑` / `↓` | Navigate services |
| `s` | Start selected service |
| `x` | Stop selected service |
| `r` | Restart selected service |
| `e` | Enable selected service |
| `d` | Disable selected service |
| `q` | Quit application |

## Service Directories

The application scans these directories for services:

- `/run/runit/service` - Primary service directory
- `/etc/runit/runsvdir/default` - Default runsvdir services
- `/etc/runit/sv` - Service definitions
- `/etc/runit` - Additional configurations

## Features

- View all available and enabled runit services
- Display service status with color coding:
  - Green: Running
  - Red: Down
  - Yellow: Finishing
- Show enabled/disabled state for each service
- Display process ID for running services
- Start, stop, and restart services
- Enable and disable services (creates/removes symlinks)
- Keyboard-driven interface

## Building Documentation

Generate HTML documentation:

```bash
cargo doc --open
```

## Project Structure

```
runitgui/
├── Cargo.toml
└── src/
    ├── main.rs        # Entry point
    ├── app.rs         # Application state and logic
    ├── ui.rs          # TUI rendering
    ├── models/
    │   └── service.rs # Service data model
    └── commands/
        └── runit.rs   # sv command wrapper
```
