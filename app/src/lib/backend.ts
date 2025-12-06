/**
 * WASM Backend for Web and Tauri
 * 
 * This module provides the game backend using WebAssembly.
 * The same WASM module runs in both browser and Tauri environments,
 * simplifying the architecture to a single code path.
 */

import type { FrontendState, GameSettings, SoundEvent, GameVariant } from './game-store.svelte';

// Backend interface
export interface IBackendAdapter {
    getGameState(): Promise<FrontendState>;
    getSettings(): Promise<GameSettings>;
    updateSettings(settings: GameSettings): Promise<FrontendState>;
    goToSettings(): Promise<FrontendState>;
    startNewGame(variant: GameVariant): Promise<FrontendState>;
    submitAnswer(answer: string): Promise<FrontendState>;
    nextLevel(): Promise<FrontendState>;
    resetGame(): Promise<FrontendState>;
    consumeSound(): Promise<SoundEvent | null>;
}

// WASM Backend - runs Rust code via WebAssembly (used for ALL platforms)
class WasmBackend implements IBackendAdapter {
    private engine: import('./wasm-pkg/letterlanders_wasm').WasmGameEngine | null = null;
    private initPromise: Promise<void> | null = null;

    private async ensureInitialized(): Promise<import('./wasm-pkg/letterlanders_wasm').WasmGameEngine> {
        if (this.engine) return this.engine;

        if (!this.initPromise) {
            this.initPromise = (async () => {
                // Dynamic import of the WASM module
                const wasm = await import('./wasm-pkg/letterlanders_wasm');
                await wasm.default(); // Initialize the WASM module
                this.engine = new wasm.WasmGameEngine();
            })();
        }

        await this.initPromise;
        return this.engine!;
    }

    async getGameState(): Promise<FrontendState> {
        const engine = await this.ensureInitialized();
        return engine.get_game_state() as FrontendState;
    }

    async getSettings(): Promise<GameSettings> {
        const engine = await this.ensureInitialized();
        return engine.get_settings() as GameSettings;
    }

    async updateSettings(settings: GameSettings): Promise<FrontendState> {
        const engine = await this.ensureInitialized();
        return engine.update_settings(settings) as FrontendState;
    }

    async goToSettings(): Promise<FrontendState> {
        const engine = await this.ensureInitialized();
        return engine.go_to_settings() as FrontendState;
    }

    async startNewGame(variant: GameVariant): Promise<FrontendState> {
        const engine = await this.ensureInitialized();
        return engine.start_new_game(variant) as FrontendState;
    }

    async submitAnswer(answer: string): Promise<FrontendState> {
        const engine = await this.ensureInitialized();
        return engine.submit_answer(answer.charAt(0)) as FrontendState;
    }

    async nextLevel(): Promise<FrontendState> {
        const engine = await this.ensureInitialized();
        return engine.next_level() as FrontendState;
    }

    async resetGame(): Promise<FrontendState> {
        const engine = await this.ensureInitialized();
        return engine.reset_game() as FrontendState;
    }

    async consumeSound(): Promise<SoundEvent | null> {
        const engine = await this.ensureInitialized();
        const sound = engine.consume_sound();
        return sound === null ? null : (sound as SoundEvent);
    }
}

// Singleton backend instance
let backend: WasmBackend | null = null;

export function getBackend(): Promise<IBackendAdapter> {
    if (!backend) {
        console.log('[Backend] Using WASM backend (Rust in WebAssembly)');
        backend = new WasmBackend();
    }
    return Promise.resolve(backend);
}
