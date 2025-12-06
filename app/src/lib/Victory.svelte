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
        background: linear-gradient(45deg, #FFD700, #FFA500);
        color: #fff;
    }

    .content {
        text-align: center;
        background: rgba(0,0,0,0.2);
        padding: 4rem;
        border-radius: 2rem;
        backdrop-filter: blur(10px);
    }

    h1 {
        font-size: 4rem;
        margin-bottom: 1rem;
        text-transform: uppercase;
        letter-spacing: 5px;
    }

    .score {
        font-size: 2rem;
        margin-bottom: 3rem;
    }

    button {
        padding: 1rem 3rem;
        font-size: 1.5rem;
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
