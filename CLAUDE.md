# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Ryn is a terminal-based game written in Rust (edition 2024) using the `crossterm` crate for terminal rendering. It runs a game loop at 24 FPS with diff-based rendering.

## Commands

- **Build:** `cargo build`
- **Run:** `cargo run`
- **Test:** `cargo test`
- **Single test:** `cargo test <test_name>`
- **Check (fast compile check):** `cargo check`

## Architecture

The game uses a **Screen** trait (`src/lib.rs`) as its core abstraction. Each screen receives input and elapsed time via `update()` — returning `true` means **quit** — and produces a frame via `produce_frame()`.

The game loop in `run_game()` drives the current screen, handling input polling, update, and render each tick. Screens are swapped via `Box<dyn Screen>`.

**Frame rendering** (`src/render.rs`): `Renderer` takes a `Frame<W, H>` (a 2D array of `Tile` indexed as `[x][y]`) and only writes tiles that changed from the previous frame to the terminal.

**Key types:**
- `Tile` — a character + color
- `Frame<W, H>` — `[[Tile; H]; W]`, column-major (x is the outer index)
- `Screen` — trait for game screens (title, overworld, etc.)

Grid dimensions are constants `WIDTH=100`, `HEIGHT=54` in `src/lib.rs`.

**Map files** (`initial_map/`): Loaded at startup by `location::load_locations`. `map.txt` defines the tile grid (`#`=Wall, `=`=Table, anything else=Floor). `map.json` defines entities to spawn. Both are required at startup.

## Style

- Prefer early returns / `continue` with `let ... else` in loops over nesting with `if let`.
- Each ECS system and its related components live in their own module under `src/overworld/` (e.g., `movement.rs` has `Position`, `Velocity`, and `MovementSystem`; `renderable.rs` has `Renderable`).
