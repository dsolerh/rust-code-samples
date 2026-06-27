# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

japlearn is a terminal UI (TUI) application for Japanese language learning, built with Rust using the ratatui framework. It uses Rust edition 2024.

## Build & Run Commands

- **Build:** `cargo build`
- **Run:** `cargo run`
- **Check:** `cargo check`
- **Run tests:** `cargo test`
- **Run single test:** `cargo test <test_name>`

## Key Dependencies

- **ratatui** — TUI framework for rendering widgets and managing terminal state
- **crossterm** — cross-platform terminal input/event handling (backend for ratatui)
- **color-eyre** — error reporting and propagation

## Architecture

The app follows ratatui's standard event loop pattern:

- `App` — top-level state holder; runs the main loop (`draw` → `handle_events` → repeat until `exit`)
- Custom widgets (e.g., `StringField`) implement ratatui's `Widget` trait for rendering and handle their own key events via dedicated `on_key_press` methods
- Esc exits the application; all other key routing flows through `App::handle_key_event`
