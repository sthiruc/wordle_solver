# Wordle Solver (Rust)

A command-line Wordle solver written in Rust that incrementally narrows down valid answers and assists in choosing optimal guesses.

---

## Features

- Loads and processes a dictionary of 5-letter words  
- Correct Wordle feedback scoring (handles duplicate letters properly)  
- Filters candidate words based on guess history  
- Interactive CLI for entering guesses and feedback  
- Input validation (ensures guesses are valid dictionary words)  
- Displays remaining possible answers after each round  
- Tracks and displays guess history  
- Unit tests for core logic  

---

## Demo

```text
Wordle Solver
Enter guesses and feedback patterns.
Pattern format: 0 = gray, 1 = yellow, 2 = green
Type 'quit' to exit.

Enter guess: crane
Enter pattern: 00100

Remaining candidates: 5
adapt
aloud
vault
...