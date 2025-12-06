<script lang="ts">
    import { game } from "./game-store.svelte";

    let { level, score, passed } = $props<{ level: number, score: number, passed: boolean }>();
    
    function onContinue() {
        game.nextLevel();
    }
</script>

<div class="overlay" class:passed={passed}>
    <div class="content">
        <h1>{passed ? "LEVEL COMPLETE!" : "LEVEL FAILED"}</h1>
        <p class="score">Score: {score}/5</p> <!-- Hardcoded /5 for now, could be passed in -->
        
        <button onclick={onContinue}>
            {passed ? "Next Level" : "Try Again"}
        </button>
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
        align-items: center;
        justify-content: center;
        background: rgba(0,0,0,0.9);
        z-index: 90;
    }

    .content {
        text-align: center;
        color: white;
    }

    h1 {
        font-size: 3rem;
        color: #ef4444;
    }

    .passed h1 {
        color: #4ade80;
    }

    .score {
        font-size: 2rem;
        margin-bottom: 2rem;
    }

    button {
        padding: 1rem 2rem;
        font-size: 1.5rem;
        cursor: pointer;
        background: white;
        border: none;
        border-radius: 1rem;
    }
</style>
