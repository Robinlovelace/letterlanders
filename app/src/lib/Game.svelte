<script lang="ts">
    import { game } from "./game-store.svelte";
    import Rocket from "./Rocket.svelte";
    import { fly, scale } from "svelte/transition";
    import { quintOut } from "svelte/easing";

    let session = $derived(game.state.session);
    let status = $derived(game.state.status);

    let rocketState = $derived.by(() => {
        if (typeof status === 'object' && 'Feedback' in status) {
            return status.Feedback.success ? 'success' : 'failure';
        }
        return 'idle';
    });

    // Handle clicks
    function handleOptionClick(option: string) {
        game.submitAnswer(option);
    }
</script>

{#if session}
    <div class="game-container">
        <div class="hud">
            <div class="hud-panel left">
                <span class="label">ALTITUDE</span>
                <span class="value">{session.current_level}</span>
            </div>
            <div class="hud-panel right">
                <span class="label">FUEL</span>
                <span class="value">{session.score}</span>
            </div>
            {#if session.level_time_limit}
                <div class="hud-panel center warning">
                    <span class="label">TIME</span>
                    <span class="value">BOSS</span>
                </div>
            {/if}
        </div>

        <div class="challenge-area">
            <Rocket state={rocketState} />
        </div>

        <div class="options-grid">
            {#each session.options as option, i (option + i)} <!-- Keyed by option+index to trigger transition on change -->
                <button 
                    class="option-card" 
                    onclick={() => handleOptionClick(option)}
                    in:fly={{ y: 200, duration: 400, delay: i * 50, easing: quintOut }}
                    out:scale={{ duration: 200 }}
                >
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
        /* Transparent background to show starfield */
        padding: 1rem;
        box-sizing: border-box;
    }

    .hud {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        padding: 0.5rem;
        margin-bottom: 1rem;
    }

    .hud-panel {
        background: rgba(16, 30, 60, 0.8);
        border: 2px solid #4488ff;
        border-radius: 8px;
        padding: 0.5rem 1rem;
        display: flex;
        flex-direction: column;
        align-items: center;
        min-width: 80px;
        box-shadow: 0 0 15px rgba(68, 136, 255, 0.3);
    }

    .hud-panel.warning {
        border-color: #ff3300;
        box-shadow: 0 0 15px rgba(255, 51, 0, 0.3);
    }

    .label {
        font-size: 0.7rem;
        color: #88ccff;
        letter-spacing: 2px;
        font-weight: bold;
    }

    .value {
        font-size: 1.5rem;
        font-weight: bold;
        color: white;
        font-family: monospace;
    }

    .challenge-area {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        position: relative;
    }

    .options-grid {
        display: flex;
        flex-wrap: wrap;
        gap: 1.5rem;
        justify-content: center;
        padding-bottom: 3rem;
        max-width: 800px;
        margin: 0 auto;
    }

    .option-card {
        width: 90px;
        height: 90px;
        font-size: 3rem;
        background: linear-gradient(135deg, #2b3a55 0%, #1a253a 100%);
        border: 2px solid #5588aa;
        border-radius: 50%; /* Planet shape */
        color: white;
        cursor: pointer;
        transition: all 0.2s ease;
        box-shadow: 0 4px 10px rgba(0,0,0,0.5), inset 0 2px 5px rgba(255,255,255,0.1);
        display: flex;
        align-items: center;
        justify-content: center;
        font-family: 'Chalkboard SE', 'Comic Sans MS', sans-serif;
        text-shadow: 0 2px 4px rgba(0,0,0,0.5);
    }

    .option-card:hover {
        transform: translateY(-5px) scale(1.05);
        border-color: #88ccff;
        box-shadow: 0 10px 20px rgba(0,0,0,0.5), 0 0 15px rgba(136, 204, 255, 0.4);
    }

    .option-card:active {
        transform: scale(0.95);
    }
    
    /* Mobile optimization */
    @media (max-width: 600px) {
        .option-card {
            width: 75px;
            height: 75px;
            font-size: 2.2rem;
        }
    }
</style>