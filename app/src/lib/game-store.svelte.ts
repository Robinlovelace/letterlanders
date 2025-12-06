import { invoke } from "@tauri-apps/api/core";

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

// Svelte 5 Runes Store
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

    constructor() {
        this.syncState();
        this.loadSettings();
        // Poll for sound every 100ms
        setInterval(() => this.checkSound(), 100);
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
            const newState = await invoke<FrontendState>("get_game_state");
            this.processState(newState);
        } catch (e) {
            console.error("Failed to sync state", e);
        }
    }

    async loadSettings() {
        try {
            this.settings = await invoke<GameSettings>("get_settings");
        } catch (e) {
            console.error("Failed to load settings", e);
        }
    }

    async saveSettings(newSettings: GameSettings) {
        try {
            this.settings = newSettings;
            const newState = await invoke<FrontendState>("update_settings", { settings: newSettings });
            this.processState(newState);
        } catch (e) {
            console.error("Failed to save settings", e);
        }
    }

    async goToSettings() {
        const newState = await invoke<FrontendState>("go_to_settings");
        this.processState(newState);
    }

    async checkSound() {
        try {
            const sound = await invoke<SoundEvent | null>("consume_sound");
            if (sound) {
                this.lastSound = sound;
            }
        } catch (e) {
            console.error("Sound error", e);
        }
    }

    async startGame(variant: GameVariant) {
        const newState = await invoke<FrontendState>("start_new_game", { variantStr: variant });
        this.processState(newState);
    }

    async submitAnswer(answer: string) {
        const newState = await invoke<FrontendState>("submit_answer", { answer: answer.charAt(0) });
        this.processState(newState);
    }

    async nextLevel() {
        const newState = await invoke<FrontendState>("next_level");
        this.processState(newState);
    }

    async reset() {
        const newState = await invoke<FrontendState>("reset_game");
        this.processState(newState);
    }
}

export const game = new GameStore();
