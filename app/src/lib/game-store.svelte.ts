import { getBackend, type IBackendAdapter } from "./backend";

// Define types locally matching Rust types
export type GameVariant = "Numbers" | "Letters";
export type InputMethod = "DirectKeyboard" | "ArrowSelection" | "Hybrid";

export interface GameSettings {
    feedback_duration_seconds: number;
    show_target_visual: boolean;
    input_method: InputMethod;
    start_level: number;
}

export type SoundEvent =
    | "None"
    | "PlaySuccess"
    | "PlayFailure"
    | { SayPrompt: string }
    | "GameStart"
    | "LevelComplete";

export type GameStatus =
    | "Menu"
    | { Settings: { message: string | null } }
    | "Playing"
    | { Feedback: { success: boolean, message: string } }
    | { LevelComplete: { level: number, score: number, passed: boolean } }
    | { SessionComplete: { score: number } };

export interface SessionState {
    variant: GameVariant;
    current_level: number;
    current_question_index: number;
    total_questions: number;
    score: number;
    total_score: number;
    target: string;
    options: string[];
    level_time_limit: number | null;
    level_elapsed_time: number;
}

export interface FrontendState {
    status: GameStatus;
    session: SessionState | null;
}

// Svelte 5 Runes Store with Backend Adapter Pattern
class GameStore {
    state = $state<FrontendState>({ status: "Menu", session: null });
    settings = $state<GameSettings>({
        feedback_duration_seconds: 2,
        show_target_visual: false,
        input_method: "Hybrid",
        start_level: 1
    });
    lastSound = $state<SoundEvent | null>(null);
    feedbackTimer: number | null = null;

    // Backend adapter - initialized lazily
    private backendPromise: Promise<IBackendAdapter>;
    private backend: IBackendAdapter | null = null;

    constructor() {
        // Initialize backend adapter (Tauri or WASM based on environment)
        this.backendPromise = getBackend();
        this.init();
    }

    private async init() {
        try {
            this.backend = await this.backendPromise;
            await this.syncState();
            await this.loadSettings();
            // Poll for sound every 100ms
            setInterval(() => this.checkSound(), 100);

            // Game Tick Loop (100ms)
            let lastTime = 0;
            setInterval(async () => {
                const now = Date.now();
                if (lastTime === 0) {
                    lastTime = now;
                    return;
                }
                const dt = (now - lastTime) / 1000;
                lastTime = now;

                // Only tick if playing to avoid unnecessary calls
                // Although backend handles status check, we save overhead here
                if (this.state.status === "Playing") {
                    await this.tick(dt);
                }
            }, 100);
        } catch (e) {
            console.error("Failed to initialize backend", e);
        }
    }

    private async getBackendOrWait(): Promise<IBackendAdapter> {
        if (this.backend) return this.backend;
        return this.backendPromise;
    }

    // Centralized method to handle state updates and side effects (like timers)
    processState(newState: FrontendState) {
        // Handle Auto-Advance for Feedback
        if (newState.status && typeof newState.status === 'object' && 'Feedback' in newState.status) {
            if (!this.feedbackTimer) {
                // Start timer if not already running
                // Use duration from settings
                const duration = this.settings.feedback_duration_seconds * 1000;
                this.feedbackTimer = setTimeout(() => {
                    this.nextLevel();
                    this.feedbackTimer = null;
                }, duration);
            }
        } else {
            // Clear timer if we moved away from Feedback
            if (this.feedbackTimer) {
                clearTimeout(this.feedbackTimer);
                this.feedbackTimer = null;
            }
        }
        this.state = newState;
    }

    async syncState() {
        try {
            const backend = await this.getBackendOrWait();
            const newState = await backend.getGameState();
            this.processState(newState);
        } catch (e) {
            console.error("Failed to sync state", e);
        }
    }

    async loadSettings() {
        try {
            const backend = await this.getBackendOrWait();
            this.settings = await backend.getSettings();
        } catch (e) {
            console.error("Failed to load settings", e);
        }
    }

    async saveSettings(newSettings: GameSettings) {
        try {
            const backend = await this.getBackendOrWait();
            this.settings = newSettings;
            const newState = await backend.updateSettings(newSettings);
            this.processState(newState);
        } catch (e) {
            console.error("Failed to save settings", e);
        }
    }

    async goToSettings() {
        const backend = await this.getBackendOrWait();
        const newState = await backend.goToSettings();
        this.processState(newState);
    }

    async checkSound() {
        try {
            const backend = await this.getBackendOrWait();
            const sound = await backend.consumeSound();
            if (sound) {
                this.lastSound = sound;
            }
        } catch (e) {
            // Silently ignore - sound polling errors are not critical
        }
    }

    async startGame(variant: GameVariant) {
        const backend = await this.getBackendOrWait();
        const newState = await backend.startNewGame(variant);
        this.processState(newState);
    }

    async submitAnswer(answer: string) {
        const backend = await this.getBackendOrWait();
        const newState = await backend.submitAnswer(answer);
        this.processState(newState);
    }

    async nextLevel() {
        const backend = await this.getBackendOrWait();
        const newState = await backend.nextLevel();
        this.processState(newState);
    }

    async reset() {
        const backend = await this.getBackendOrWait();
        const newState = await backend.resetGame();
        this.processState(newState);
    }

    async tick(dt: number) {
        const backend = await this.getBackendOrWait();
        const newState = await backend.tick(dt);
        this.processState(newState);
    }
}

export const game = new GameStore();
