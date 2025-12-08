<script lang="ts">
    import { game } from "./game-store.svelte";
</script>

<div class="about-container">
    <div class="content">
        <h1>About LetterLanders</h1>

        <p>
            LetterLanders is a cross-platform educational game designed to teach
            letter and number recognition to children. It generalizes the
            concept of the "Digits" app, gamifying the learning process with a
            "Rocket Landing" theme.
        </p>

        <p>
            This project is inspired by <a
                href="https://github.com/foxtrotdev/learn-digits"
                target="_blank"
                rel="noreferrer">Learn Digits</a
            >.
        </p>

        <h2>Credits</h2>
        <p>Built with Rust, Svelte, and Tauri.</p>

        <button onclick={() => game.reset()}>Back</button>
    </div>
</div>

<style>
    .about-container {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100vh;
        width: 100vw;
        color: white;
        background: transparent;
    }

    .content {
        background: rgba(16, 30, 60, 0.9);
        padding: 2rem;
        border-radius: 1rem;
        border: 2px solid #5588aa;
        max-width: 600px;
        text-align: center;
        box-shadow: 0 0 20px rgba(0, 0, 0, 0.5);
    }

    h1 {
        margin-top: 0;
        color: #ffd700;
        margin-bottom: 1.5rem;
    }

    h2 {
        color: #88ccff;
        margin-top: 2rem;
        margin-bottom: 1rem;
    }

    p {
        line-height: 1.6;
        margin-bottom: 1rem;
        font-size: 1.1rem;
    }

    a {
        color: #44aaff;
        text-decoration: none;
        font-weight: bold;
    }

    a:hover {
        text-decoration: underline;
        color: #88ccff;
    }

    button {
        margin-top: 2rem;
        padding: 0.8rem 2rem;
        font-size: 1.2rem;
        background: #5588aa;
        color: white;
        border: none;
        border-radius: 0.5rem;
        cursor: pointer;
        transition: background 0.2s;
    }

    button:hover {
        background: #447799;
    }
</style>
