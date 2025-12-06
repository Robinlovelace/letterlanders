'''import { browser } from "$app/environment";
import { GameEngine } from "core";
import type { GameSettings, GameStatus, GameVariant, SessionState, SoundEvent } from "core";

class GameStore {
    engine: GameEngine | null = $state(null);
    
    // Read-only state proxies
    status: GameStatus | null = $state(null);
    session: SessionState | null = $state(null);
    settings: GameSettings | null = $state(null);
    lastSound: SoundEvent | null = $state(null);
    
    feedbackTimer: number | null = null;
    
    constructor() {
        if (browser) {
            this.engine = new GameEngine();
            this.syncState();
        }
    }
    
    // Make sure we sync state between the engine and the store
    syncState() {
        if (!this.engine) return;
        
        this.status = this.engine.status;
        this.session = this.engine.session;
        this.settings = this.engine.settings;
        this.lastSound = this.engine.consume_sound();

        this.handleFeedback();
    }
    
    handleFeedback() {
        if (this.status && typeof this.status === 'object' && 'Feedback' in this.status) {
            if (this.feedbackTimer) clearTimeout(this.feedbackTimer);
            
            const duration = this.settings?.feedback_duration_seconds ?? 2 * 1000;
            this.feedbackTimer = setTimeout(() => {
                this.nextLevel();
                this.feedbackTimer = null;
            }, duration);
        } else {
            if (this.feedbackTimer) {
                clearTimeout(this.feedbackTimer);
                this.feedbackTimer = null;
            }
        }
    }
    
    async saveSettings(newSettings: GameSettings) {
        if (!this.engine) return;
        this.engine.settings = newSettings;
        this.engine.status = { Settings: { message: "Settings saved!" } };
        this.syncState();
        
        setTimeout(() => this.reset(), 1000);
    }
    
    async goToSettings() {
        if (!this.engine) return;
        this.engine.status = { Settings: { message: null } };
        this.syncState();
    }
    
    async startGame(variant: GameVariant) {
        if (!this.engine) return;
        this.engine.start_game(variant);
        this.syncState();
    }
    
    async submitAnswer(answer: string) {
        if (!this.engine || !this.session) return;
        this.engine.submit_answer(answer.charAt(0));
        this.syncState();
    }

    async submitCurrentSelection() {
        if (!this.engine || !this.session) return;
        this.engine.submit_current_selection();
        this.syncState();
    }

    async moveSelection(delta: number) {
        if (!this.engine || !this.session) return;
        this.engine.move_selection(delta);
        this.syncState();
    }
    
    async nextLevel() {
        if (!this.engine || !this.session) return;
        
        // This is a bit of a hack. The frontend shouldn't be making this decision.
        if (this.status && typeof this.status === 'object' && ('Feedback' in this.status || 'LevelComplete' in this.status)) {
            if ('Feedback' in this.status && (this.status.Feedback as any).success) {
                this.engine.next_level();
            } else if ('LevelComplete' in this.status && (this.status.LevelComplete as any).passed) {
                this.engine.advance_to_next_level_or_retry();
            } else {
                // On failure, we just go back to playing state without advancing
                this.engine.status = "Playing";
            }
        } else {
            this.engine.next_level();
        }
        
        this.syncState();
    }

    async advanceToNextLevelOrRetry() {
        if (!this.engine) return;
        this.engine.advance_to_next_level_or_retry();
        this.syncState();
    }
    
    async reset() {
        if (!this.engine) return;
        this.engine = new GameEngine();
        this.syncState();
    }
}

export const game = new GameStore();''