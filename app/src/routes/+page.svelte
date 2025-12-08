<script lang="ts">
    import { game } from "$lib/game-store.svelte";
    import Menu from "$lib/Menu.svelte";
    import Game from "$lib/Game.svelte";
    import Feedback from "$lib/Feedback.svelte";
    import Victory from "$lib/Victory.svelte";
    import LevelComplete from "$lib/LevelComplete.svelte";
    import AudioPlayer from "$lib/AudioPlayer.svelte";
    import Settings from "$lib/Settings.svelte";
    import About from "$lib/About.svelte";

    let status = $derived(game.state.status);
    let audioPlayer: ReturnType<typeof AudioPlayer>;

    // Monitor for timer ticks in Boss Mode
    let lastSecond = -1;
    $effect(() => {
        const session = game.state.session;
        if (session && session.level_time_limit) {
            const timeLeft = Math.ceil(
                session.level_time_limit - session.level_elapsed_time,
            );
            if (
                timeLeft !== lastSecond &&
                timeLeft <= session.level_time_limit
            ) {
                lastSecond = timeLeft;
                if (audioPlayer && timeLeft > 0) {
                    audioPlayer.playTick(timeLeft <= 3);
                }
            }
        } else {
            lastSecond = -1;
        }
    });
</script>

<div class="starfield"></div>
<AudioPlayer bind:this={audioPlayer} />

{#if status === "Menu"}
    <Menu />
{:else if status === "About"}
    <About />
{:else if typeof status === "object" && "Settings" in status}
    <Settings />
{:else}
    {#if status === "Playing" || (typeof status === "object" && ("Feedback" in status || "LevelComplete" in status))}
        <Game />
    {/if}

    {#if typeof status === "object" && "Feedback" in status}
        <!-- Only show feedback overlay if NOT a successful boss level (to let explosion play) -->
        {#if !(status.Feedback.success && game.state.session?.current_level >= 4)}
            <Feedback {...status.Feedback} />
        {/if}
    {/if}

    {#if typeof status === "object" && "LevelComplete" in status}
        <LevelComplete {...status.LevelComplete} />
    {/if}

    {#if typeof status === "object" && "SessionComplete" in status}
        <Victory />
    {/if}
{/if}

<style>
    :global(*) {
        box-sizing: border-box;
    }

    :global(html, body) {
        margin: 0;
        padding: 0;
        overflow-x: hidden;
        max-width: 100vw;
    }

    :global(body) {
        font-family: "Comic Sans MS", "Chalkboard SE", sans-serif;
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
        background: radial-gradient(
                white,
                rgba(255, 255, 255, 0.2) 2px,
                transparent 3px
            ),
            radial-gradient(
                white,
                rgba(255, 255, 255, 0.15) 1px,
                transparent 2px
            ),
            radial-gradient(
                white,
                rgba(255, 255, 255, 0.1) 2px,
                transparent 3px
            );
        background-size:
            550px 550px,
            350px 350px,
            250px 250px;
        background-position:
            0 0,
            40px 60px,
            130px 270px;
        animation: backgroundMove 60s linear infinite;
    }

    @keyframes backgroundMove {
        from {
            background-position:
                0 0,
                40px 60px,
                130px 270px;
        }
        to {
            background-position:
                550px 550px,
                390px 410px,
                680px 820px;
        }
    }
</style>
