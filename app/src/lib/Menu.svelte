<script lang="ts">
    import { game } from "./game-store.svelte";
</script>

<div class="menu">
    <h1>LetterLanders</h1>
    <p class="subtitle">Land Safely on the Moon!</p>
    
    <div class="buttons">
        <button onclick={() => game.startGame("Numbers")}>
            <span class="icon">123</span>
            Play Numbers
        </button>
        <button onclick={() => game.startGame("Letters")}>
            <span class="icon">ABC</span>
            Play Letters
        </button>
    </div>
</div>

<style>
    .menu {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100vh;
        background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
        color: white;
    }

    h1 {
        font-size: 4rem;
        margin-bottom: 0.5rem;
        color: #ffd700;
        text-shadow: 0 0 10px rgba(255, 215, 0, 0.5);
    }

    .subtitle {
        font-size: 1.5rem;
        color: #a0a0a0;
        margin-bottom: 3rem;
    }

    .buttons {
        display: flex;
        gap: 2rem;
    }

    button {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 2rem;
        font-size: 1.5rem;
        background: rgba(255, 255, 255, 0.1);
        border: 2px solid rgba(255, 255, 255, 0.2);
        border-radius: 1rem;
        color: white;
        cursor: pointer;
        transition: all 0.2s;
        width: 200px;
    }

    button:hover {
        transform: translateY(-5px);
        background: rgba(255, 255, 255, 0.2);
        border-color: #ffd700;
    }

    .icon {
        font-size: 3rem;
        margin-bottom: 1rem;
    }
</style>
