# Commando Developer Guide

This guide is aimed at developers looking to contribute to or understand the internal workings of the Commando CLI tool.

---

## Table of Contents

1. [Project Overview](#project-overview)
2. [Codebase Structure](#codebase-structure)
3. [Core Modules and Their Roles](#core-modules-and-their-roles)
4. [Flow and Interaction](#flow-and-interaction)
5. [How to Contribute](#how-to-contribute)
6. [Contact](#contact)

---

## Project Overview

Commando is a CLI tool designed to assist developers in writing conventional commit messages interactively. The project is built with Rust, leveraging [Ratatui] for terminal UI rendering and follows modular design principles.

[Ratatui]: https://ratatui.rs

---

## Codebase Structure

The project is organized into modular components, each handling a specific responsibility:

```
src
├── app
│   └── mod.rs
├── components
│   ├── mod.rs
│   └── text_block.rs
├── features
├── main.rs
├── screen
│   ├── manager.rs
│   ├── mod.rs
│   └── my_initial_screen.rs
├── state
│   ├── in_memory.rs
│   ├── manager.rs
│   └── mod.rs
└── ui
    ├── mod.rs
    └── render.rs
```

---

## Core Modules and Their Roles

### 1. `app/`
- **Purpose**: Manages the main application loop and integrates other modules.
- **Key File**: `mod.rs`
- **Highlights**:
  - Initializes the application state and screen.
  - Controls the event loop.

---

### 2. `components/`
- **Purpose**: Contains reusable UI components.
- **Example Component**: `TextBlock` for rendering text.
- **Interaction**: Components are managed by the `UIRenderer`.

---

### 3. `screen/`
- **Purpose**: Handles individual screens and their transitions.
- **Key File**: `manager.rs`
- **Highlights**:
  - Each screen implements the `Screen` trait.
  - Manages transitions via `ScreenManager`.

Refer to [Application Structure Diagram](./board.md) for more details.

---

### 4. `state/`
- **Purpose**: Manages application data and state.
- **Example**: `InMemoryState` stores key-value pairs.
- **Highlights**:
  - Implements the `State` trait for flexibility.

---

### 5. `ui/`
- **Purpose**: Facilitates UI rendering and event delegation.
- **Key File**: `render.rs`
- **Highlights**:
  - Manages components via `UIRenderer`.

---

### 6. `main.rs`
- **Purpose**: The entry point for initializing and running the application.
- **Highlights**:
  - Sets up the terminal interface.
  - Calls the `App`'s `run` method.

---

## Flow and Interaction

1. **Startup**:  
   - The `main.rs` initializes the terminal, state, and the starting screen.

2. **Event Loop**:  
   - The `App` module manages the loop, delegating events to the current screen.

3. **Screen Management**:  
   - The `ScreenManager` transitions between screens and passes events.

4. **Rendering**:  
   - `UIRenderer` manages components and renders them via the terminal.

---

## How to Contribute

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/Phastboy/commando
   ```

2. **Set Up Development Environment**:
   - Install Rust and its tooling.
   - Run `cargo build` to ensure everything compiles.

3. **Add Features or Fix Bugs**:
   - Follow the existing modular design.
   - Update or create modules as necessary.

4. **Submit a Pull Request**:
   - Ensure code passes all tests.
   - Write meaningful commit messages.

---

## Contact

For questions or collaboration, reach out to:  
**Hammed Anuoluwapo Pelumi**  
- Email: stationphast@gmail.com  
- GitHub: [Phastboy](https://github.com/Phastboy)
