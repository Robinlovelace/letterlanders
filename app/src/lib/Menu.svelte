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

    <button class="settings-btn" onclick={() => game.goToSettings()}>
        Settings
    </button>
</div>

<style>
    .menu {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100vh;
        background: transparent; /* Reveal starfield */
        color: white;
    }

    h1 {
        font-size: clamp(2.5rem, 8vw, 4rem); /* Responsive font size */
        margin-bottom: 0.5rem;
        color: #ffd700;
        text-shadow: 0 0 10px rgba(255, 215, 0, 0.5);
        text-align: center;
    }

    .subtitle {
        font-size: clamp(1rem, 4vw, 1.5rem);
        color: #a0a0a0;
        margin-bottom: 3rem;
        text-align: center;
    }

    .buttons {
        display: flex;
        gap: 2rem;
        flex-wrap: wrap;
        justify-content: center;
        width: 100%;
        padding: 0 1rem;
        box-sizing: border-box;
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
        flex: 1 1 140px; /* Grow, shrink, basis */
        max-width: 220px;
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

    .settings-btn {
        margin-top: 2rem;
        width: auto;
        padding: 1rem 2rem;
        font-size: 1.2rem;
        background: transparent;
        border: 1px solid rgba(255, 255, 255, 0.3);
    }
</style>
