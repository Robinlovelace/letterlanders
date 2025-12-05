#!/bin/bash
echo "Generating sound assets..."

# Core Feedback Sounds
espeak -w assets/sounds/success.wav "Great job!"
espeak -w assets/sounds/failure.wav "Oops, try again."
espeak -w assets/sounds/complete.wav "Session complete! You are amazing."

# Number Prompts (1-9)
for i in {1..9}; do
    espeak -w "assets/sounds/prompts/$i.wav" "Choose $i"
done

# Letter Prompts (A-Z)
for char in {A..Z}; do
    espeak -w "assets/sounds/prompts/$char.wav" "Choose $char"
done

echo "Done! Assets generated in assets/sounds/"
