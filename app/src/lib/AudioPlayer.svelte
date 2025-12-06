<script lang="ts">
    import { game } from "./game-store.svelte";

    // Track if user has interacted (for autoplay policy)
    let userHasInteracted = $state(false);
    
    // Queue sounds until user interacts
    let soundQueue: any[] = $state([]);
    
    // Enable audio on first click anywhere
    function enableAudio() {
        if (!userHasInteracted) {
            userHasInteracted = true;
            // Play any queued sounds
            for (const sound of soundQueue) {
                playSound(sound);
            }
            soundQueue = [];
        }
    }
    
    // Listen for clicks globally
    if (typeof window !== 'undefined') {
        window.addEventListener('click', enableAudio, { once: true });
        window.addEventListener('keydown', enableAudio, { once: true });
    }

    // We need to react to sound events
    $effect(() => {
        if (game.lastSound) {
            if (userHasInteracted) {
                playSound(game.lastSound);
            } else {
                soundQueue = [...soundQueue, game.lastSound];
            }
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
                const audio = new Audio(`/sounds/${filename}`);
                await audio.play();
            } catch (e) {
                console.error("Failed to play audio:", e);
            }
        }
    }
</script>
