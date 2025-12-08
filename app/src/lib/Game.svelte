<script lang="ts">
    import { game } from "./game-store.svelte";
    import Rocket from "./Rocket.svelte";
    import Boss from "./Boss.svelte";
    import { fly, scale } from "svelte/transition";
    import { quintOut } from "svelte/easing";

    let session = $derived(game.state.session);
    let status = $derived(game.state.status);

    let rocketState: "idle" | "success" | "failure" = $derived.by(() => {
        if (typeof status === "object" && "Feedback" in status) {
            return status.Feedback.success ? "success" : "failure";
        }
        return "idle";
    });

    let progress = $derived.by(() => {
        if (typeof status === "object" && "LevelComplete" in status) {
            return 1;
        }
        if (session && session.total_questions > 0) {
            return session.current_question_index / session.total_questions;
        }
        return 0;
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
                <span class="label">LEVEL</span>
                <span class="value">{session.current_level}</span>
            </div>
            {#if session.current_level >= 4}
                <div class="boss-level-text">
                    <span class="scrolling-text">BOSS LEVEL!</span>
                </div>
            {/if}
            <div class="hud-panel right">
                <span class="label">ALTITUDE</span>
                <span class="value">{Math.round(5000 * (1 - progress))}</span>
            </div>
        </div>

        <div class="challenge-area">
            {#if session.current_level >= 4}
                <div class="boss-wrapper">
                    <Boss state={rocketState} />
                    {#if session.level_time_limit}
                        <div
                            class="timer {session.level_time_limit -
                                session.level_elapsed_time <=
                            3
                                ? 'urgent'
                                : ''}"
                        >
                            {Math.ceil(
                                Math.max(
                                    0,
                                    session.level_time_limit -
                                        session.level_elapsed_time,
                                ),
                            )}
                        </div>
                    {/if}
                </div>
            {:else}
                <Rocket
                    state={rocketState}
                    level={session.current_level}
                    {progress}
                />
            {/if}
        </div>

        <div class="options-grid">
            {#each session.options as option, i (option + i)}
                <!-- Keyed by option+index to trigger transition on change -->
                <button
                    class="option-card"
                    onclick={() => handleOptionClick(option)}
                    in:fly={{
                        y: 200,
                        duration: 400,
                        delay: i * 50,
                        easing: quintOut,
                    }}
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
        position: relative;
        z-index: 100;
    }

    .boss-level-text {
        position: absolute;
        top: 0;
        left: 50%;
        transform: translateX(-50%);
        width: 100%;
        text-align: center;
        overflow: hidden;
        pointer-events: none;
    }

    .scrolling-text {
        display: inline-block;
        font-family: "Chalkboard SE", "Comic Sans MS", sans-serif;
        font-size: 2rem; /* Smaller text */
        font-weight: 900;
        color: #ff3300;
        text-shadow:
            2px 2px 0 #fff,
            -2px -2px 0 #fff,
            2px -2px 0 #fff,
            -2px 2px 0 #fff;
        animation: pulse-grow 1s infinite alternate;
        padding: 0.5rem;
    }

    @keyframes pulse-grow {
        0% {
            transform: scale(1);
        }
        100% {
            transform: scale(1.1);
        }
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

    .boss-wrapper {
        position: relative;
        width: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .timer {
        position: absolute;
        top: -40%; /* Move it EVEN HIGHER */
        left: 50%;
        transform: translateX(-50%); /* Center horizontally only */
        font-size: 4rem; /* Smaller than 6rem */
        font-weight: 900;
        color: rgba(255, 255, 255, 0.9);
        text-shadow: 0 0 10px rgba(0, 0, 0, 0.8);
        font-family: monospace;
        z-index: 20;
        pointer-events: none;
    }

    .timer.urgent {
        color: #ff3300;
        animation: timer-pulse 0.5s infinite;
    }

    @keyframes timer-pulse {
        0%,
        100% {
            transform: translateX(-50%) scale(1);
        }
        50% {
            transform: translateX(-50%) scale(1.1);
        }
    }

    .options-grid {
        display: flex;
        flex-wrap: wrap;
        gap: 1.5rem;
        justify-content: center;
        padding-bottom: 3rem;
        max-width: 800px;
        margin: 0 auto;
        position: relative;
        z-index: 20;
    }

    .option-card {
        width: 110px;
        height: 110px;
        font-size: 3.5rem;
        background: linear-gradient(135deg, #2b3a55 0%, #1a253a 100%);
        border: 2px solid #5588aa;
        border-radius: 50%; /* Planet shape */
        color: white;
        cursor: pointer;
        transition: all 0.2s ease;
        box-shadow:
            0 4px 10px rgba(0, 0, 0, 0.5),
            inset 0 2px 5px rgba(255, 255, 255, 0.1);
        display: flex;
        align-items: center;
        justify-content: center;
        font-family: "Chalkboard SE", "Comic Sans MS", sans-serif;
        text-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
    }

    .option-card:hover {
        transform: translateY(-5px) scale(1.05);
        border-color: #88ccff;
        box-shadow:
            0 10px 20px rgba(0, 0, 0, 0.5),
            0 0 15px rgba(136, 204, 255, 0.4);
    }

    .option-card:active {
        transform: scale(0.95);
    }

    @media (max-width: 600px) {
        .option-card {
            width: 90px;
            height: 90px;
            font-size: 2.8rem;
        }
    }
</style>
