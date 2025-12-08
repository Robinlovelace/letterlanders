<script lang="ts">
    import { game } from "./game-store.svelte";

    let { message, success } = $props<{ message: string, success: boolean }>();

    // Auto-advance logic is handled by Rust backend timer or we can add a manual "Next" button
    // But since our Rust backend has a 'wait' logic, we might just show this until state changes
</script>

<div class="feedback-overlay" class:success={success}>
    <div class="content">
        <h1>{success ? "Great Job!" : "Oops!"}</h1>
        <p>{message}</p>
    </div>
</div>

<style>
    .feedback-overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        background: rgba(0,0,0,0.8);
        z-index: 100;
    }

    .success .content {
        color: #4ade80;
    }

    .content {
        background: white;
        padding: clamp(1.5rem, 5vw, 3rem);
        border-radius: 1.5rem;
        text-align: center;
        color: #ef4444;
        animation: popIn 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
        max-width: calc(100vw - 2rem);
        margin: 0 1rem;
        box-sizing: border-box;
    }

    h1 {
        font-size: clamp(2rem, 8vw, 3rem);
        margin: 0;
        word-wrap: break-word;
    }

    p {
        font-size: clamp(1rem, 4vw, 1.5rem);
        word-wrap: break-word;
    }

    @keyframes popIn {
        from { transform: scale(0.5); opacity: 0; }
        to { transform: scale(1); opacity: 1; }
    }
</style>
