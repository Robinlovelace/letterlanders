<script lang="ts">
    import { game } from "$lib/game-store.svelte";
    import Menu from "$lib/Menu.svelte";
    import Game from "$lib/Game.svelte";
    import Feedback from "$lib/Feedback.svelte";
    import Victory from "$lib/Victory.svelte";
    import LevelComplete from "$lib/LevelComplete.svelte";
    import AudioPlayer from "$lib/AudioPlayer.svelte";

    let status = $derived(game.state.status);
</script>

<AudioPlayer />

{#if status === "Menu"}
    <Menu />
{:else if status === "Playing" || (typeof status === "object" && "Feedback" in status)}
    <Game />
    {#if typeof status === "object" && "Feedback" in status}
        <Feedback {...status.Feedback} />
    {/if}
{:else if typeof status === "object" && "LevelComplete" in status}
    <LevelComplete {...status.LevelComplete} />
{:else if typeof status === "object" && "SessionComplete" in status}
    <Victory />
{:else}
    <div class="fallback">
        <p>Unknown state: {JSON.stringify(status)}</p>
    </div>
{/if}

<style>
    :global(body) {
        margin: 0;
        font-family: 'Comic Sans MS', 'Chalkboard SE', sans-serif;
        overflow: hidden;
    }
    
    .fallback {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100vh;
        color: white;
        background: #333;
    }
</style>
