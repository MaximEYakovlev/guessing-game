# Guess the Number Game

This is a simple command-line game written in Rust. The program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message and exit.

## Features
- Random number generation within a specified range.
- Feedback on guesses: "Too big!", "Too small!", "You win!".
- Simple command-line interface.

---

## Prerequisites
To compile and run the project, you need:
- [Rust](https://www.rust-lang.org/tools/install) (version 1.65 or later recommended).
- A terminal or command prompt.

---

## Getting Started

### Clone the Repository
```bash
git clone git@github.com:MaximEYakovlev/guessing-game.git
cd guessing-game
```

### Build the Project
To compile the project, run:
```bash
cargo build
```
This will generate the compiled files in the target/debug directory.

### Run the Project
To start the game, run:
```bash
cargo run
```

### Example
    Guess the number!
    Please input your guess.
    50
    You guessed: 50
    Too small!
    Please input your guess.
    75
    You guessed: 75
    Too big!
    Please input your guess.
    63
    You guessed: 63
    You win!


