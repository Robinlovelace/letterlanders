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

    constructor() {}

    processState(newState: FrontendState) {
        if (newState.status && typeof newState.status === 'object' && 'Feedback' in newState.status) {
            if (this.feedbackTimer) {
                clearTimeout(this.feedbackTimer);
            }
            const duration = this.settings.feedback_duration_seconds * 1000;
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
        this.state = newState;
    }

    async saveSettings(newSettings: GameSettings) {
        this.settings = newSettings;
        this.processState({ ...this.state, status: { Settings: { message: "Settings saved!" } } });
        setTimeout(() => this.reset(), 1000);
    }

    async goToSettings() {
        this.processState({ ...this.state, status: { Settings: { message: null } } });
    }

    async startGame(variant: GameVariant) {
        this.processState({
            status: "Playing",
            session: {
                variant,
                current_level: 1,
                current_question_index: 0,
                total_questions: 5,
                score: 0,
                total_score: 0,
                target: variant === "Letters" ? "A" : "1",
                options: variant === "Letters" ? ["A", "B", "C", "D"] : ["1", "2", "3", "4"],
                level_time_limit: null,
                level_elapsed_time: 0,
            }
        });
    }

    async submitAnswer(answer: string) {
        if (!this.state.session) return;

        const success = this.state.session.target.toUpperCase() === answer.toUpperCase();
        let newScore = this.state.session.score;
        if (success) {
            newScore += 10;
        }

        this.processState({
            ...this.state,
            session: { ...this.state.session, score: newScore },
            status: { Feedback: { success, message: success ? "Correct!" : "Try again!" } }
        });
    }

    async nextLevel() {
        if (!this.state.session) return;

        if (this.state.session.current_question_index < this.state.session.total_questions - 1) {
            const nextIndex = this.state.session.current_question_index + 1;
            const nextTarget = this.state.session.variant === "Letters" 
                ? String.fromCharCode(65 + nextIndex) 
                : (nextIndex + 1).toString();
            
            this.processState({
                ...this.state,
                session: {
                    ...this.state.session,
                    current_question_index: nextIndex,
                    target: nextTarget,
                },
                status: "Playing"
            });

        } else {
            this.processState({
                ...this.state,
                status: { LevelComplete: { level: this.state.session.current_level, score: this.state.session.score, passed: this.state.session.score > 30 } }
            });
        }
    }

    async reset() {
        this.processState({ status: "Menu", session: null });
    }
    
    async syncState() {}
    async loadSettings() {}
    async checkSound() {}
}

export const game = new GameStore();
