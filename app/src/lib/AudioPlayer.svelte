<script lang="ts">
    import { game } from "./game-store.svelte";
    import { resolveResource } from '@tauri-apps/api/path';
    import { convertFileSrc } from '@tauri-apps/api/core';

    // We need to react to sound events
    $effect(() => {
        if (game.lastSound) {
            playSound(game.lastSound);
            game.lastSound = null; // Clear it so we don't play it again
        }
    });

    async function playSound(event: any) {
        let filename = "";
        
        if (event === "PlaySuccess") filename = "success.wav";
        else if (event === "PlayFailure") filename = "failure.wav";
        else if (event === "LevelComplete") filename = "complete.wav";
        else if (typeof event === "object" && "SayPrompt" in event) {
            filename = `prompts/${event.SayPrompt}.wav`;
        }

        if (filename) {
            try {
                // In Tauri v2, we use the asset protocol.
                // For local dev, we can put assets in 'public' or handle them via Rust.
                // Since we put sounds in `assets/sounds` which is outside `app`, 
                // we might need to configure Tauri to serve them or copy them to `app/public`.
                // For now, let's assume we need to fetch them properly.
                
                // Quick fix: Copy sounds to app/public/sounds for the webview to access easily
                const audio = new Audio(`/sounds/${filename}`);
                await audio.play();
            } catch (e) {
                console.error("Failed to play audio", e);
            }
        }
    }
</script>
