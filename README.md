# Limit Timer

A simple and elegant timer that always stays on top of other windows.

## Features

- Transparent and movable window
- Always on top
- Turns red when less than 15 minutes remain
- Display format: "XX h XX min XX s"

## Installation

1. Make sure you have Rust installed on your system
2. Clone this repository
3. Build the project with:
```bash
cargo build --release
```

## Usage

To launch the timer with the default duration of 60 minutes:
```bash
./target/release/limit_timer
```

To specify a custom duration (in minutes):
```bash
./target/release/limit_timer 30  # For a 30-minute timer
```

## Start at Windows Startup

To automatically launch the timer at Windows startup:

1. Press `Windows + R`
2. Type `shell:startup` and press Enter
3. Create a shortcut to the executable in this folder
4. Edit the shortcut properties to add the duration argument if needed

## Notes

- The window cannot be resized or closed with a button (use Task Manager if needed)
- The timer turns red when less than 15 minutes remain 