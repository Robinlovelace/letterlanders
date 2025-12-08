<script lang="ts">
    import { game } from "./game-store.svelte";
    
    // Extract props from the status object if needed, but we can access session directly
    let session = $derived(game.state.session);
</script>

<div class="victory-container">
    <div class="content">
        <h1>YOU DID IT!</h1>
        <p class="score">Total Score: {session?.total_score || 0}</p>
        <button onclick={() => game.reset()}>Back to Menu</button>
    </div>
</div>

<style>
    .victory-container {
        height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
        background: rgba(255, 215, 0, 0.1); /* Subtle gold tint */
        color: #fff;
        padding: 1rem;
        box-sizing: border-box;
    }

    .content {
        text-align: center;
        background: linear-gradient(45deg, rgba(255, 215, 0, 0.9), rgba(255, 165, 0, 0.9)); /* Gold/Orange gradient panel */
        padding: clamp(2rem, 5vw, 4rem);
        border-radius: 1.5rem;
        backdrop-filter: blur(10px);
        box-shadow: 0 0 50px rgba(255, 215, 0, 0.5);
        animation: popIn 0.5s cubic-bezier(0.175, 0.885, 0.32, 1.275);
        max-width: calc(100vw - 2rem);
        box-sizing: border-box;
    }

    @keyframes popIn {
        from { transform: scale(0.5); opacity: 0; }
        to { transform: scale(1); opacity: 1; }
    }

    h1 {
        font-size: clamp(2rem, 8vw, 4rem);
        margin-bottom: 1rem;
        text-transform: uppercase;
        letter-spacing: clamp(2px, 1vw, 5px);
        word-wrap: break-word;
    }

    .score {
        font-size: clamp(1.25rem, 5vw, 2rem);
        margin-bottom: clamp(1.5rem, 4vw, 3rem);
    }

    button {
        padding: clamp(0.75rem, 2vw, 1rem) clamp(1.5rem, 4vw, 3rem);
        font-size: clamp(1rem, 3vw, 1.5rem);
        border: none;
        border-radius: 50px;
        background: white;
        color: #FFA500;
        font-weight: bold;
        cursor: pointer;
        box-shadow: 0 4px 15px rgba(0,0,0,0.2);
        transition: transform 0.2s;
    }

    button:hover {
        transform: scale(1.05);
    }
</style>
