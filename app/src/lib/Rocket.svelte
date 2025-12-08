<script lang="ts">
    export let state: "idle" | "success" | "failure" = "idle";
    export let level: number = 1;
    export let progress: number = 0;
</script>

<div class="scene">
    <div class="rocket-positioner" style="transform: translateY(calc(-40vh + {progress} * (75vh - 75px)))">
        <div class="rocket-container {state} {progress >= 1 ? 'landed' : ''}">
            <svg viewBox="0 0 100 100" class="rocket" overflow="visible">
                <defs>
                <linearGradient id="bodyGradient" x1="0%" y1="0%" x2="100%" y2="0%">
                    <stop offset="0%" style="stop-color:#f5f5f5" />
                    <stop offset="50%" style="stop-color:#ffffff" />
                    <stop offset="100%" style="stop-color:#e0e0e0" />
                </linearGradient>
                <filter id="shadow" x="-20%" y="-20%" width="140%" height="140%">
                    <feDropShadow dx="1" dy="1" stdDeviation="1" flood-opacity="0.3"/>
                </filter>
            </defs>
    
            <!-- Flames -->
            {#if progress < 1}
            <g class="flames" transform="translate(0, -2)">
                <path d="M40 82 Q50 110 60 82" fill="#ff9800" opacity="0.9">
                    <animate attributeName="d" values="M40 82 Q50 105 60 82; M35 82 Q50 125 65 82; M40 82 Q50 105 60 82" dur="0.15s" repeatCount="indefinite" />
                </path>
                <path d="M45 82 Q50 100 55 82" fill="#ffeb3b" opacity="0.9">
                    <animate attributeName="d" values="M45 82 Q50 92 55 82; M43 82 Q50 105 57 82; M45 82 Q50 92 55 82" dur="0.15s" repeatCount="indefinite" />
                </path>
            </g>
            {/if}
    
            <!-- Fins -->
            <!-- Left -->
            <path d="M32 75 L15 95 L35 85 Z" fill="#d32f2f" stroke="#b71c1c" stroke-width="1" stroke-linejoin="round"/>
            <!-- Right -->
            <path d="M68 75 L85 95 L65 85 Z" fill="#d32f2f" stroke="#b71c1c" stroke-width="1" stroke-linejoin="round"/>
            <!-- Center (Back) -->
            <path d="M50 80 L50 92" stroke="#b71c1c" stroke-width="4" stroke-linecap="round"/>
    
            <!-- Body -->
            <path d="M30 80 Q25 40 50 5 Q75 40 70 80 Z" fill="url(#bodyGradient)" stroke="#9e9e9e" stroke-width="1" filter="url(#shadow)"/>
    
            <!-- Nose Cone (Red tip) -->
            <path d="M36.5 35 Q50 5 63.5 35 Q50 40 36.5 35 Z" fill="#d32f2f" />
    
            <!-- Window -->
            <circle cx="50" cy="50" r="12" fill="#29b6f6" stroke="#37474f" stroke-width="2.5"/>
            <!-- Glare -->
            <ellipse cx="54" cy="46" rx="3" ry="2" fill="white" opacity="0.7" transform="rotate(-45 54 46)"/>
            
            <!-- Rivets/Seams -->
            <circle cx="50" cy="25" r="1" fill="#bdbdbd"/>
            <circle cx="50" cy="70" r="1" fill="#bdbdbd"/>
        </svg>
    </div>
    </div>
    
    <div class="surface">
        <div class="crater c1"></div>
        <div class="crater c2"></div>
        <div class="crater c3"></div>
    </div>
</div>

<style>
    .scene {
        width: 100%;
        height: 100%;
        position: relative;
        display: flex;
        justify-content: center;
        align-items: center;
        overflow: visible;
    }

    .rocket-positioner {
        transition: transform 1s cubic-bezier(0.34, 1.56, 0.64, 1);
        will-change: transform;
        z-index: 10;
    }

    .rocket-container {
        width: 150px;
        height: 150px;
        transition: transform 0.5s cubic-bezier(0.175, 0.885, 0.32, 1.275);
    }

    .rocket {
        width: 100%;
        height: 100%;
        filter: drop-shadow(0 0 10px rgba(255, 200, 0, 0.3));
    }

    .surface {
        position: absolute;
        bottom: -15vh;
        left: -50%;
        width: 200%;
        height: 20vh;
        background: linear-gradient(to bottom, #795548, #3e2723);
        border-radius: 50% 50% 0 0;
        border-top: 8px solid #a1887f;
        box-shadow: 0 -10px 30px rgba(0,0,0,0.5);
        z-index: 1;
    }

    .crater {
        position: absolute;
        background: rgba(0,0,0,0.2);
        border-radius: 50%;
        box-shadow: inset 2px 2px 5px rgba(0,0,0,0.4);
    }

    .c1 { width: 40px; height: 20px; left: 45%; top: 30px; }
    .c2 { width: 20px; height: 10px; left: 55%; top: 60px; }
    .c3 { width: 60px; height: 30px; left: 35%; top: 80px; }

    /* Idle Animation */
    .idle .rocket {
        animation: hover 2s ease-in-out infinite;
    }

    .landed .rocket {
        animation: none;
    }

    @keyframes hover {
        0%, 100% { transform: translateY(0); }
        50% { transform: translateY(-10px); }
    }

    /* Success Animation */
    .success {
        animation: stabilize 1s ease-out forwards;
    }

    @keyframes stabilize {
        0% { transform: scale(1); filter: brightness(1); }
        50% { transform: scale(1.1); filter: brightness(1.2); }
        100% { transform: scale(1); filter: brightness(1); }
    }

    .success .flames {
        filter: drop-shadow(0 0 10px #ff6600);
        opacity: 1;
    }

    /* Failure Animation */
    .failure .rocket {
        animation: shake 0.5s cubic-bezier(.36,.07,.19,.97) both;
    }

        @keyframes shake {
        10%, 90% { transform: translate3d(-2px, 0, 0) rotate(-2deg); }
        20%, 80% { transform: translate3d(4px, 0, 0) rotate(2deg); }
        30%, 50%, 70% { transform: translate3d(-6px, 0, 0) rotate(-4deg); }
        40%, 60% { transform: translate3d(6px, 0, 0) rotate(4deg); }
    }
</style>
