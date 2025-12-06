#!/bin/bash
# Download assets from the latest GitHub release (v0.0.2)
VERSION="v0.0.2"
BASE_URL="https://github.com/Robinlovelace/letterlanders/releases/download/$VERSION"

mkdir -p app/static/sounds/prompts

echo "Downloading Sound Assets for $VERSION..."

# Core Sounds
curl -L -o app/static/sounds/success.wav "$BASE_URL/success.wav"
curl -L -o app/static/sounds/failure.wav "$BASE_URL/failure.wav"
curl -L -o app/static/sounds/complete.wav "$BASE_URL/complete.wav"

# Prompts
for i in {1..9}; do
    curl -L -o "app/static/sounds/prompts/$i.wav" "$BASE_URL/$i.wav"
done

for char in {A..Z}; do
    curl -L -o "app/static/sounds/prompts/$char.wav" "$BASE_URL/$char.wav"
done

echo "Assets downloaded to app/static/sounds/"
