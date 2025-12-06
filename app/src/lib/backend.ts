/**
 * Backend Adapter Pattern for Tauri + WASM Support
 * 
 * This module provides a unified interface for the game backend,
 * allowing the frontend to work with either:
 * - Tauri (desktop/mobile): Uses IPC to communicate with Rust process
 * - WASM (web): Runs Rust code directly in the browser via WebAssembly
 */

import type { FrontendState, GameSettings, SoundEvent, GameVariant } from './game-store.svelte';

// Backend interface that both adapters implement
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

// Tauri Backend Adapter - communicates with Rust via IPC
class TauriBackend implements IBackendAdapter {
    private invoke: typeof import('@tauri-apps/api/core').invoke;

    constructor(invoke: typeof import('@tauri-apps/api/core').invoke) {
        this.invoke = invoke;
    }

    async getGameState(): Promise<FrontendState> {
        return this.invoke<FrontendState>("get_game_state");
    }

    async getSettings(): Promise<GameSettings> {
        return this.invoke<GameSettings>("get_settings");
    }

    async updateSettings(settings: GameSettings): Promise<FrontendState> {
        return this.invoke<FrontendState>("update_settings", { settings });
    }

    async goToSettings(): Promise<FrontendState> {
        return this.invoke<FrontendState>("go_to_settings");
    }

    async startNewGame(variant: GameVariant): Promise<FrontendState> {
        return this.invoke<FrontendState>("start_new_game", { variantStr: variant });
    }

    async submitAnswer(answer: string): Promise<FrontendState> {
        return this.invoke<FrontendState>("submit_answer", { answer: answer.charAt(0) });
    }

    async nextLevel(): Promise<FrontendState> {
        return this.invoke<FrontendState>("next_level");
    }

    async resetGame(): Promise<FrontendState> {
        return this.invoke<FrontendState>("reset_game");
    }

    async consumeSound(): Promise<SoundEvent | null> {
        return this.invoke<SoundEvent | null>("consume_sound");
    }
}

// WASM Backend Adapter - runs Rust code directly in the browser
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

// Detect environment and create appropriate backend
export async function createBackend(): Promise<IBackendAdapter> {
    // Check if we're in a Tauri environment
    const isTauri = typeof window !== 'undefined' && '__TAURI__' in window;

    if (isTauri) {
        console.log('[Backend] Using Tauri adapter (IPC to Rust process)');
        const { invoke } = await import('@tauri-apps/api/core');
        return new TauriBackend(invoke);
    } else {
        console.log('[Backend] Using WASM adapter (Rust in browser)');
        return new WasmBackend();
    }
}

// Export a singleton promise for the backend
let backendPromise: Promise<IBackendAdapter> | null = null;

export function getBackend(): Promise<IBackendAdapter> {
    if (!backendPromise) {
        backendPromise = createBackend();
    }
    return backendPromise;
}
