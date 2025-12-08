<script lang="ts">
    import { game } from "./game-store.svelte";

    let { level, score, passed } = $props<{
        level: number;
        score: number;
        passed: boolean;
    }>();

    function onContinue() {
        game.nextLevel();
    }
</script>

<div class="overlay" class:passed>
    <div class="content">
        <h1>{passed ? "LEVEL COMPLETE!" : "LEVEL FAILED"}</h1>
        <p class="score">Score: {score}/5</p>
        <!-- Hardcoded /5 for now, could be passed in -->

        <div class="actions">
            <button class="primary" onclick={onContinue}>
                {passed ? "Next Level" : "Try Again"}
            </button>
            <button class="secondary" onclick={() => game.reset()}>
                Main Menu
            </button>
        </div>
    </div>
</div>

<style>
    .overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        display: flex;
        align-items: flex-start; /* Align to top to leave space for rocket */
        padding-top: 15vh;
        padding-left: 1rem;
        padding-right: 1rem;
        justify-content: center;
        background: rgba(
            0,
            0,
            0,
            0.3
        ); /* More transparent to see the landing */
        z-index: 90;
        backdrop-filter: blur(2px);
        box-sizing: border-box;
    }

    .content {
        text-align: center;
        color: white;
        max-width: calc(100vw - 2rem);
        box-sizing: border-box;
    }

    h1 {
        font-size: clamp(1.75rem, 7vw, 3rem);
        color: #ef4444;
        word-wrap: break-word;
    }

    .passed h1 {
        color: #4ade80;
    }

    .score {
        font-size: clamp(1.25rem, 5vw, 2rem);
        margin-bottom: clamp(1rem, 3vw, 2rem);
    }

    .actions {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        align-items: center;
    }

    button {
        padding: clamp(0.75rem, 2vw, 1rem) clamp(1.5rem, 4vw, 2rem);
        font-size: clamp(1rem, 3vw, 1.5rem);
        cursor: pointer;
        border: none;
        border-radius: 1rem;
        width: 100%;
        max-width: 300px;
    }

    .primary {
        background: white;
        color: #000;
        font-weight: bold;
    }

    .secondary {
        background: rgba(255, 255, 255, 0.2);
        color: white;
        border: 2px solid rgba(255, 255, 255, 0.5);
    }
</style>
