#!/bin/bash
echo "Generating sound assets..."

# Core Feedback Sounds
espeak -w app/static/sounds/success.wav "Great job!"
espeak -w app/static/sounds/failure.wav "Oops, try again."
espeak -w app/static/sounds/complete.wav "Session complete! You are amazing."

# Number Prompts (1-9)
for i in {1..9}; do
    espeak -w "app/static/sounds/prompts/$i.wav" "Choose $i"
done

# Letter Prompts (A-Z)
for char in {A..Z}; do
    espeak -w "app/static/sounds/prompts/$char.wav" "Choose $char"
done

# Letter Teams Prompts
teams=("th" "sh" "ch" "wh" "ph" "ng" "ck" "qu")
for team in "${teams[@]}"; do
    espeak -w "app/static/sounds/prompts/$team.wav" "Choose $team"
done

echo "Done! Assets generated in assets/sounds/"
