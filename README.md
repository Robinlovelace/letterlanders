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

# Download sound assets (required for audio)
# Option 1: Using GitHub CLI (downloads from latest release)
mkdir -p static/sounds/prompts
gh release download --repo robinlovelace/letterlanders --pattern "*.wav" --dir static/sounds/prompts

# Option 2: Copy from assets folder (if available locally)
cp -r ../assets/sounds static/

npm run tauri dev
```

## Android Signing Setup

1. Initialize the Android project once: `cd app && npm run tauri android init`.
2. Generate a private keystore locally (git-ignored):

Edit `.create-keystore.sh` defaults if needed, then run:

```bash
chmod +x .create-keystore.sh
./.create-keystore.sh
```

The script writes `app/src-tauri/gen/android/app/release-keystore.jks` to match the CI workflow.
3. Add GitHub repository secrets for the workflow:
    - `ANDROID_KEYSTORE`: `base64 -w0 app/src-tauri/gen/android/app/release-keystore.jks`
    - `ANDROID_KEYSTORE_PASSWORD`: the store password you chose
    - `ANDROID_KEY_ALIAS`: the alias you chose
    - `ANDROID_KEY_PASSWORD`: the key password (use the store password if shared)
    - Optional deploy: `GOOGLE_PLAY_SERVICE_ACCOUNT_JSON` (service account JSON for Play uploads)
4. Trigger the `Android Build` workflow (`main` push or `workflow_dispatch`); the release job produces a signed `.aab` artifact.

## Project Structure

```text
letterlanders/
├── Cargo.toml          # Workspace config
├── core/               # Game Logic Library
├── tui/                # Terminal Frontend
└── app/                # Tauri + Svelte Frontend
```