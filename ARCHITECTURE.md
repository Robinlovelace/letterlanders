# LetterLanders Architecture

## 1. High-Level Overview
LetterLanders is a cross-platform educational game designed to teach letter recognition. The architecture follows a **Clean Architecture** approach, strictly separating the Core Business Logic from the Interface/Presentation layers. This ensures the game mechanics remain identical whether running in a Linux terminal, a web browser, or as an Android app.

## 2. Technology Stack Recommendation
Based on your requirements for modularity, performance, TUI support, and cross-platform deployment (including Play Store):

*   **Language:** **Rust** (for Core Logic & TUI) + **TypeScript** (for Web/GUI).
*   **Core Logic:** Rust crate (Library). Portable, type-safe, and compiles to WebAssembly (WASM) if needed.
*   **TUI (Terminal UI):** Rust using **Ratatui** (formerly tui-rs) + **Crossterm**. Best-in-class for rich terminal interfaces.
*   **GUI/Mobile:** **Tauri v2** (Rust Backend + Svelte/TypeScript Frontend). Tauri v2 specifically supports mobile targets (Android/iOS).
*   **Frontend Framework:** **Svelte**. Lightweight, reactive, and simpler than React, fitting the "simplicity" goal.

## 3. Project Structure (Rust Workspace)
The project will be organized as a Rust Workspace to manage dependencies efficiently.

```text
letterlanders/
├── Cargo.toml              # Workspace configuration
├── core/                   # Pure Game Logic (No UI code)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── state.rs        # Game State Machine (Idle, Playing, Feedback)
│   │   ├── levels.rs       # Difficulty scaling logic
│   │   └── engine.rs       # Main game loop controller
│   └── Cargo.toml
├── tui/                    # Terminal User Interface
│   ├── src/
│   │   ├── main.rs
│   │   ├── ui.rs           # Rendering logic (ASCII art rocket/moon)
│   │   └── input.rs        # Keyboard handling
│   └── Cargo.toml
└── app/                    # Tauri Application (GUI/Mobile)
    ├── src-tauri/          # Rust backend bridging Core -> Frontend
    ├── src/                # Svelte + TypeScript Frontend
    ├── package.json
    └── tauri.conf.json
```

## 4. Data Flow & State Management

### The Core State Machine
The `core` library will expose a platform-agnostic State Machine.

*   **States:**
    *   `Menu`: Selecting game variant (Letters, Numbers, etc.).
    *   `Challenge`: Waiting for user input (e.g., "Press 'A'").
    *   `Feedback(Success)`: "Rocket lands safely."
    *   `Feedback(Failure)`: "Rocket drifts/shrinks."
    *   `LevelComplete`: Session summary/Next level prompt.

### Input Handling (Decision: Direct Key Press)
*   **TUI:** Listens for character events. If the target is '3', pressing '3' triggers success.
*   **Web/Mobile:** Can support both on-screen buttons (touch) and physical keyboard (desktop).

### Interfaces
1.  **TUI:** The Rust binary imports `core`. It maps keyboard events directly to game inputs.
2.  **Web/App:** The Tauri backend wraps `core`.

## 5. Asset Management (Sound & Visuals)
*   **Audio:**
    *   *Abstracted Interface:* The Core emits `SoundEvent` enums (e.g., `PlaySuccess`, `SayPrompt('A')`).
    *   *TUI Implementation:* Uses **`rodio`** to play local asset files asynchronously.
    *   *Web Implementation:* Frontend listens for events and plays audio via HTML5 API.
*   **Visuals:**
    *   *TUI:* ASCII Art / Braille patterns for the rocket and moon.
    *   *Web:* SVG/Canvas animations.

## 6. Development Roadmap
1.  **Phase 1: The Core (Pure Rust)**
    *   Implement `GameVariant` (Letters, Numbers).
    *   Implement Session-based logic (Fixed number of questions per level).
    *   Define `SoundEvent` system.
2.  **Phase 2: The TUI (Ratatui + Rodio)**
    *   Build the terminal interface.
    *   Implement audio playback integration.
    *   verify "Press Key" interaction.
3.  **Phase 3: The GUI (Tauri + Svelte)**
    *   Initialize Tauri app.
    *   Connect Rust core to Svelte store.
    *   Create visual assets and animations.
4.  **Phase 4: Polish & Deploy**
    *   Cross-compilation for Android.
    *   Packaging for Linux/Web.
