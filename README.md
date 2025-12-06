# LetterLanders

LetterLanders is a cross-platform educational game designed to teach letter and number recognition to children. It generalizes the concept of the "Digits" app, gamifying the learning process with a "Rocket Landing" theme.

## Architecture

The project is built as a **Rust Workspace** with a Clean Architecture approach:

*   **`core/`**: Pure Rust business logic (State Machine, Level Generation). Platform-agnostic.
*   **`tui/`**: A Terminal User Interface using `ratatui` and `rodio`. Runs in your shell.
*   **`app/`**: A GUI Mobile/Desktop application using **Tauri v2** + **Svelte**.

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed design decisions.

## Prerequisites

1.  **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
2.  **Node.js**: [Install Node.js](https://nodejs.org/) (for the Tauri frontend)
3.  **System Dependencies (Linux)**:
    ```bash
    sudo apt-get install libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libasound2-dev espeak
    ```

## How to Run

### 1. Terminal Game (TUI)
The TUI is the fastest way to test game logic and play in a text-based environment.

```bash
# Run from the project root
cargo run -p letterlanders-tui
```

**Controls:**
*   `N`: Numbers Game
*   `L`: Letters Game
*   `Esc`: Quit

### 2. Graphical App (Desktop/Web)
The Tauri app provides the rich visual experience.

```bash
cd app
npm install
npm run tauri dev
```

## Project Structure

```text
letterlanders/
├── Cargo.toml          # Workspace config
├── core/               # Game Logic Library
├── tui/                # Terminal Frontend
└── app/                # Tauri + Svelte Frontend
```