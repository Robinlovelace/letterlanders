<script lang="ts">
    import { game } from "./game-store.svelte";
    import type { GameSettings } from "./game-store.svelte";

    // Create a local copy for editing
    let localSettings = $state<GameSettings>({ ...game.settings });

    function save() {
        game.saveSettings(localSettings);
    }

    function cancel() {
        game.reset(); // Or just go back to menu without saving
    }
</script>

<div class="settings-container">
    <div class="panel">
        <h1>Settings</h1>

        <div class="setting-group">
            <label>
                <span>Show Target Visual</span>
                <input type="checkbox" bind:checked={localSettings.show_target_visual} />
            </label>
        </div>

        <div class="setting-group">
            <label>
                <span>Feedback Duration (s)</span>
                <input type="range" min="1" max="5" step="1" bind:value={localSettings.feedback_duration_seconds} />
                <span class="value">{localSettings.feedback_duration_seconds}s</span>
            </label>
        </div>

        <div class="setting-group">
            <label>
                <span>Start Level</span>
                <select bind:value={localSettings.start_level}>
                    <option value={1}>Level 1 (2 Options)</option>
                    <option value={2}>Level 2 (3 Options)</option>
                    <option value={3}>Level 3 (5 Options)</option>
                    <option value={4}>Level 4 (Boss Mode)</option>
                </select>
            </label>
        </div>

        <div class="setting-group">
            <label>
                <span>Input Method</span>
                <select bind:value={localSettings.input_method}>
                    <option value="DirectKeyboard">Keyboard Only</option>
                    <option value="ArrowSelection">Arrows Only</option>
                    <option value="Hybrid">Hybrid (Both)</option>
                </select>
            </label>
        </div>

        <div class="actions">
            <button class="cancel" onclick={cancel}>Cancel</button>
            <button class="save" onclick={save}>Save & Exit</button>
        </div>
    </div>
</div>

<style>
    .settings-container {
        height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
        background: transparent;
        color: white;
        padding: 1rem;
        box-sizing: border-box;
    }

    .panel {
        background: rgba(16, 30, 60, 0.95);
        border: 2px solid #4488ff;
        padding: clamp(1rem, 4vw, 2rem);
        border-radius: 1rem;
        width: 100%;
        max-width: 400px;
        box-shadow: 0 0 20px rgba(68, 136, 255, 0.2);
        box-sizing: border-box;
    }

    h1 {
        margin-top: 0;
        text-align: center;
        color: #ffd700;
        font-size: clamp(1.5rem, 5vw, 2rem);
    }

    .setting-group {
        margin-bottom: 1.5rem;
    }

    label {
        display: flex;
        flex-wrap: wrap;
        justify-content: space-between;
        align-items: center;
        font-size: clamp(0.9rem, 3vw, 1.2rem);
        gap: 0.5rem;
    }

    select, input[type="range"] {
        padding: 0.5rem;
        font-size: 1rem;
        border-radius: 0.5rem;
        max-width: 100%;
    }

    input[type="checkbox"] {
        width: 1.5rem;
        height: 1.5rem;
    }

    .actions {
        display: flex;
        justify-content: space-between;
        margin-top: 2rem;
    }

    button {
        padding: 0.8rem 1.5rem;
        font-size: 1rem;
        border: none;
        border-radius: 0.5rem;
        cursor: pointer;
        transition: transform 0.1s;
    }

    button:active {
        transform: scale(0.95);
    }

    .save {
        background: #4ade80;
        color: #004d00;
        font-weight: bold;
    }

    .cancel {
        background: #ef4444;
        color: white;
    }
</style>
