# Dino Quiz 🦕

A dinosaur identification game built in Rust with the Bevy game engine.

## What it does

A dinosaur image appears on screen and you have to type its name. Points are awarded based on how specific your answer is:

- **3 points** — exact full name 
- **2 points** — close partial name 
- **1 point** — correct taxonomic group 
- **0 points** — incorrect or skipped

18 dinosaurs total, maximum score 54.

## Controls

| Key | Action |
|-----|--------|
| Type | Enter your answer |
| Enter | Submit answer |
| ↓ | Show a hint |
| → | Next dinosaur (after answering) |
| ← | Go back to review a previous answer |

Type `skip` and press Enter to skip a dinosaur (you'll be asked to confirm, type yes or no).

## How to run

You'll need Rust installed. If you don't have it, get it from [rustup.rs](https://rustup.rs).

## Paste this into your terminal:
cd dino-game

cargo run

**The first run will take a few minutes to compile. Subsequent runs are much faster! Promise!**

## Built with

- [Rust](https://www.rust-lang.org/)
- [Bevy 0.15](https://bevyengine.org/)

