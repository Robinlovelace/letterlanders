<script lang="ts">
    export let state: "idle" | "success" | "failure" = "idle";
</script>

<div class="rocket-container {state}">
    <svg viewBox="0 0 100 100" class="rocket" overflow="visible">
        <!-- Flames (Animated) -->
        <g class="flames">
            <path d="M30 80 Q50 110 70 80" fill="#ff6600" opacity="0.8">
                <animate attributeName="d" values="M35 80 Q50 100 65 80; M30 80 Q50 120 70 80; M35 80 Q50 100 65 80" dur="0.2s" repeatCount="indefinite" />
            </path>
            <path d="M40 80 Q50 100 60 80" fill="#ffff00" opacity="0.9">
                <animate attributeName="d" values="M42 80 Q50 95 58 80; M40 80 Q50 105 60 80; M42 80 Q50 95 58 80" dur="0.2s" repeatCount="indefinite" />
            </path>
        </g>
        
        <!-- Fins -->
        <path d="M20 80 L20 60 L30 50 L30 80 Z" fill="#cc3300"/>
        <path d="M80 80 L80 60 L70 50 L70 80 Z" fill="#cc3300"/>
        <path d="M45 80 L55 80 L55 50 L45 50 Z" fill="#cc3300"/>

        <!-- Body -->
        <ellipse cx="50" cy="50" rx="25" ry="40" fill="#eeeeee" stroke="#999" stroke-width="2"/>
        
        <!-- Window -->
        <circle cx="50" cy="40" r="12" fill="#66ccff" stroke="#333" stroke-width="2"/>
        <circle cx="50" cy="40" r="10" fill="#99ddff" opacity="0.5"/>
        <circle cx="54" cy="36" r="3" fill="white" opacity="0.8"/>

        <!-- Nose Cone -->
        <path d="M25 50 Q50 0 75 50" fill="#ff3300" />
    </svg>
</div>

<style>
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

    /* Idle Animation */
    .idle .rocket {
        animation: hover 2s ease-in-out infinite;
    }

    @keyframes hover {
        0%, 100% { transform: translateY(0); }
        50% { transform: translateY(-10px); }
    }

    /* Success Animation */
    .success {
        animation: blastOff 1s ease-in forwards;
    }

    @keyframes blastOff {
        0% { transform: translateY(0) scale(1); }
        20% { transform: translateY(10px) scale(0.9); }
        100% { transform: translateY(-500px) scale(0.5); opacity: 0; }
    }

    .success .flames {
        filter: drop-shadow(0 0 20px #ff6600);
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
