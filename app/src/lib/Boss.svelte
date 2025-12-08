<script lang="ts">
    export let state: "idle" | "success" | "failure" = "idle";
</script>

<div class="boss-container {state}">
    <svg viewBox="0 0 100 100" class="boss-svg" overflow="visible">
        <defs>
            <linearGradient id="armor" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#4a5568" />
                <stop offset="100%" style="stop-color:#2d3748" />
            </linearGradient>
            <filter id="visorGlow">
                <feGaussianBlur stdDeviation="1.5" result="coloredBlur" />
                <feMerge>
                    <feMergeNode in="coloredBlur" />
                    <feMergeNode in="SourceGraphic" />
                </feMerge>
            </filter>
        </defs>

        <!-- Hover Jets -->
        <g class="jets">
            <ellipse cx="30" cy="85" rx="5" ry="2" fill="#ed8936" opacity="0.8">
                <animate
                    attributeName="opacity"
                    values="0.8;0.4;0.8"
                    dur="0.2s"
                    repeatCount="indefinite"
                />
            </ellipse>
            <ellipse cx="70" cy="85" rx="5" ry="2" fill="#ed8936" opacity="0.8">
                <animate
                    attributeName="opacity"
                    values="0.8;0.4;0.8"
                    dur="0.2s"
                    repeatCount="indefinite"
                    begin="0.1s"
                />
            </ellipse>
        </g>

        <!-- Main Body -->
        <path
            d="M25 25 L75 25 L85 40 L85 75 L75 85 L25 85 L15 75 L15 40 Z"
            fill="url(#armor)"
            stroke="#1a202c"
            stroke-width="2"
        />

        <!-- Shoulder Pads -->
        <rect
            x="5"
            y="20"
            width="20"
            height="30"
            rx="2"
            fill="#2d3748"
            stroke="#1a202c"
            stroke-width="2"
        />
        <rect
            x="75"
            y="20"
            width="20"
            height="30"
            rx="2"
            fill="#2d3748"
            stroke="#1a202c"
            stroke-width="2"
        />

        <!-- Visor (Eye) - Made slightly taller for better visibility -->
        <rect x="30" y="32" width="40" height="14" rx="4" fill="#000" />
        <rect
            x="32"
            y="34"
            width="36"
            height="10"
            rx="2"
            fill="#f56565"
            filter="url(#visorGlow)"
        >
            <!-- Blinking/Scanning Effect -->
            <animate
                attributeName="fill"
                values="#f56565;#742a2a;#f56565"
                dur="1s"
                repeatCount="indefinite"
            />

            {#if state === "failure"}
                <animate
                    attributeName="fill"
                    values="#fff;#f56565"
                    dur="0.1s"
                    repeatCount="indefinite"
                />
            {/if}
        </rect>

        <!-- Mouth Grille -->
        <g stroke="#718096" stroke-width="2">
            <line x1="35" y1="60" x2="65" y2="60" />
            <line x1="35" y1="65" x2="65" y2="65" />
            <line x1="35" y1="70" x2="65" y2="70" />
        </g>

        <!-- Antenna -->
        <line
            x1="50"
            y1="25"
            x2="50"
            y2="5"
            stroke="#718096"
            stroke-width="2"
        />
        <circle cx="50" cy="5" r="3" fill="#f56565">
            <animate
                attributeName="fill"
                values="#f56565;#feb2b2;#f56565"
                dur="1s"
                repeatCount="indefinite"
            />
        </circle>
        <!-- Laser Beams - Positioned over the visor area -->
        <circle
            cx="40"
            cy="38"
            r="4"
            fill="#fff"
            class="laser-beam"
            style="opacity: 0"
        />
        <circle
            cx="60"
            cy="38"
            r="4"
            fill="#fff"
            class="laser-beam"
            style="opacity: 0"
        />
    </svg>
</div>

<style>
    .boss-container {
        width: 100%;
        height: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .boss-svg {
        width: 100%;
        height: 100%;
        max-width: 250px;
        max-height: 250px;
    }

    /* Idle Animation */
    .idle .boss-svg {
        animation: float 4s ease-in-out infinite;
    }

    @keyframes float {
        0%,
        100% {
            transform: translateY(0);
        }
        50% {
            transform: translateY(-5px);
        }
    }

    /* Success (Boss Hit - DEFEAT SEQUENCE) */
    .success .boss-svg {
        animation: boss-defeat 4s ease-in-out forwards;
    }

    @keyframes boss-defeat {
        0%,
        40% {
            /* Phase 1: WOBBLE - Violent shaking */
            transform: translate3d(-2px, 2px, 0) rotate(-2deg);
        }
        5%,
        35% {
            transform: translate3d(2px, -2px, 0) rotate(2deg);
        }
        10%,
        30% {
            transform: translate3d(-3px, 0, 0) rotate(-4deg);
        }
        15%,
        25% {
            transform: translate3d(3px, 0, 0) rotate(4deg);
        }
        20% {
            transform: translate3d(0, 0, 0) scale(1.1) rotate(0);
            filter: hue-rotate(90deg);
        }

        40% {
            /* Phase 2: PRE-EXPLOSION - Swell and brighten */
            transform: scale(1.2) rotate(10deg);
            filter: brightness(1.5) sepia(1);
            opacity: 1;
        }
        60% {
            /* Phase 3: CRITICAL MASS - Shaking uncontrollably */
            transform: scale(0.8) rotate(-10deg);
            filter: brightness(3) invert(1);
            opacity: 1;
        }
        80% {
            /* Phase 4: EXPLOSION */
            transform: scale(20);
            filter: brightness(10) blur(10px);
            opacity: 0.5;
        }
        100% {
            /* Phase 5: GONE */
            transform: scale(30);
            opacity: 0;
        }
    }

    /* Failure (Boss Attack) */
    /* Failure (Boss Attack) */
    .failure .boss-svg {
        animation: surge 0.5s ease-out;
    }

    .failure .laser-beam {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%) scale(0.1);
        width: 20px;
        height: 20px;
        background-color: #39ff14; /* Neon Green */
        border-radius: 50%;
        box-shadow:
            0 0 10px #39ff14,
            0 0 20px #39ff14,
            0 0 40px #39ff14;
        opacity: 0;
        animation: laser-fire-fail 0.8s ease-in forwards;
    }

    /* Continuous Laser Fire (Idle) - Green Dots */
    .idle .laser-beam {
        /* Default animation (overridden by nth-of-type below) */
        animation: laser-pew-1 1.2s infinite linear;
        transform-box: fill-box;
        transform-origin: center;
        fill: #39ff14; /* Green */
    }

    /* Randomize firing patterns */
    .idle .laser-beam:nth-of-type(1) {
        animation: laser-pew-1 1.1s infinite linear;
        animation-delay: 0s;
    }
    .idle .laser-beam:nth-of-type(2) {
        animation: laser-pew-2 1.3s infinite linear;
        animation-delay: 0.6s;
    }

    @keyframes surge {
        0% {
            transform: scale(1);
        }
        50% {
            transform: scale(1.1);
        }
        100% {
            transform: scale(1);
        }
    }

    /* Direction 1: Down-Right */
    @keyframes laser-pew-1 {
        0% {
            opacity: 1;
            transform: scale(1) translate(0, 0);
            fill: #fff;
        }
        20% {
            fill: #39ff14;
        }
        100% {
            opacity: 0;
            transform: scale(3) translate(80px, 120px);
            fill: #00ff00;
        }
    }

    /* Direction 2: Down-Left */
    @keyframes laser-pew-2 {
        0% {
            opacity: 1;
            transform: scale(1) translate(0, 0);
            fill: #fff;
        }
        20% {
            fill: #39ff14;
        }
        100% {
            opacity: 0;
            transform: scale(3) translate(-80px, 100px);
            fill: #00ff00;
        }
    }

    @keyframes laser-fire-fail {
        0% {
            opacity: 1;
            transform: translate(-50%, -50%) scale(0.1);
            background-color: #fff;
        }
        20% {
            background-color: #39ff14;
        }
        100% {
            opacity: 0.8; /* Translucent */
            transform: translate(-50%, -50%) scale(50); /* Fill screen */
            background-color: #39ff14;
        }
    }
</style>
