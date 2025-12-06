<script lang="ts">
    import { game } from "$lib/game-store.svelte";
    import Menu from "$lib/Menu.svelte";
    import Game from "$lib/Game.svelte";
    import Feedback from "$lib/Feedback.svelte";
    import Victory from "$lib/Victory.svelte";
    import LevelComplete from "$lib/LevelComplete.svelte";
    import AudioPlayer from "$lib/AudioPlayer.svelte";
    import Settings from "$lib/Settings.svelte";

    let status = $derived(game.state.status);
</script>

<div class="starfield"></div>
<AudioPlayer />

{#if status === "Menu"}
    <Menu />
{:else if typeof status === "object" && "Settings" in status}
    <Settings />
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
        background-color: #0b0d17;
        color: white;
    }

    .starfield {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        z-index: -1;
        background: 
            radial-gradient(white, rgba(255,255,255,.2) 2px, transparent 3px),
            radial-gradient(white, rgba(255,255,255,.15) 1px, transparent 2px),
            radial-gradient(white, rgba(255,255,255,.1) 2px, transparent 3px);
        background-size: 550px 550px, 350px 350px, 250px 250px;
        background-position: 0 0, 40px 60px, 130px 270px;
        animation: backgroundMove 60s linear infinite;
    }

    @keyframes backgroundMove {
        from { background-position: 0 0, 40px 60px, 130px 270px; }
        to { background-position: 550px 550px, 390px 410px, 680px 820px; }
    }
    
    .fallback {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100vh;
        color: white;
        background: rgba(0,0,0,0.5);
    }
</style>
