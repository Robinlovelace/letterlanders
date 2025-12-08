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
    if (typeof window !== "undefined") {
        window.addEventListener("click", enableAudio, { once: true });
        window.addEventListener("keydown", enableAudio, { once: true });
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

    // Exposed method for simple tick sound (oscillator)
    export function playTick(urgent: boolean) {
        if (!userHasInteracted) return;
        try {
            const ctx = new (window.AudioContext ||
                (window as any).webkitAudioContext)();
            const osc = ctx.createOscillator();
            const gain = ctx.createGain();

            osc.connect(gain);
            gain.connect(ctx.destination);

            osc.type = urgent ? "square" : "sine";
            osc.frequency.value = urgent ? 880 : 440; // High pitch for urgent
            gain.gain.value = 0.1;

            osc.start();
            osc.stop(ctx.currentTime + 0.1);
        } catch (e) {
            // ignore
        }
    }
</script>
