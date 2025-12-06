#!/bin/bash
# Download assets from the latest GitHub release (v0.0.2)
VERSION="v0.0.2"

mkdir -p app/static/sounds/prompts

echo "Downloading Sound Assets for $VERSION..."

# Core Sounds
gh release download $VERSION -p "success.wav" -O app/static/sounds/success.wav --clobber
gh release download $VERSION -p "failure.wav" -O app/static/sounds/failure.wav --clobber
gh release download $VERSION -p "complete.wav" -O app/static/sounds/complete.wav --clobber

# Prompts
for i in {1..9}; do
    gh release download $VERSION -p "$i.wav" -O "app/static/sounds/prompts/$i.wav" --clobber
done

for char in {A..Z}; do
    gh release download $VERSION -p "$char.wav" -O "app/static/sounds/prompts/$char.wav" --clobber
done

echo "Assets downloaded to app/static/sounds/"
