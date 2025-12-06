<script lang="ts">
    import { game } from "./game-store.svelte";

    let session = $derived(game.state.session);
    
    // Handle clicks
    function handleOptionClick(option: string) {
        game.submitAnswer(option);
    }
</script>

{#if session}
    <div class="game-container">
        <div class="hud">
            <div class="level">Level {session.current_level}</div>
            <div class="score">Score: {session.score}</div>
            {#if session.level_time_limit}
                 <!-- Basic timer visualization, static for now as update loop is in Rust -->
                 <!-- Ideally we sync elapsed time more frequently or animate locally -->
                <div class="timer">Boss Timer!</div>
            {/if}
        </div>

        <div class="challenge-area">
            <h2>Choose:</h2>
            <!-- We hide the target visually as per default settings, 
                 but prompts are audio-based. -->
        </div>

        <div class="options-grid">
            {#each session.options as option}
                <button class="option-card" onclick={() => handleOptionClick(option)}>
                    {option}
                </button>
            {/each}
        </div>
    </div>
{/if}

<style>
    .game-container {
        height: 100vh;
        display: flex;
        flex-direction: column;
        background: #222;
        color: white;
        padding: 2rem;
    }

    .hud {
        display: flex;
        justify-content: space-between;
        font-size: 1.5rem;
        padding: 1rem;
        background: rgba(0,0,0,0.3);
        border-radius: 1rem;
    }

    .challenge-area {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    .options-grid {
        display: flex;
        flex-wrap: wrap;
        gap: 1rem;
        justify-content: center;
        margin-bottom: 2rem;
    }

    .option-card {
        width: 100px;
        height: 100px;
        font-size: 3rem;
        background: #333;
        border: 2px solid #555;
        border-radius: 1rem;
        color: white;
        cursor: pointer;
        transition: transform 0.1s;
    }

    .option-card:active {
        transform: scale(0.95);
    }
</style>
