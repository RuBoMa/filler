# filler
In this project, we implement a bot to play the Filler game.
A game where two bots compete to fill the most territory on a grid-based map (Anfield) by taking turns placing pieces of various shapes.

Each turn, a bot must place a peice on the map according to specific rules:
- The piece must touch one and only one of the bot's existing cell on the map.
- The piece cannot overlap with any of the opponent's cells or go out of bounds.
- The piece must fit entirely within the map.

This project is part of the curriculum at [01 edu school](https://01edu.ai/).
The game engine and robots are provided by the school, and we have to implement our own bot in Rust to win against the provided bots.

We have implemented a visualizer to watch the matches and analyze the game logs.


## Prerequisites
The project is designed by the school to run in a Docker container where the game_engine will run the solution bot against other bots.
- [Docker](https://www.docker.com/get-started)

If you wish to test the solution bot locally, you will need:
- [Rust and Cargo](https://rustup.rs/)

## Understanding the setup
The game_engine runs each bot as a separate process and communicates with them via standard input and output.

At the start of the game, the game_engine sends the player information to each bot:
```bash
$$$ exec p1 : [path_to_bot_executable]
```

After that, the game_engine sends the map and piece information to each bot in turns.
```bash
Anfield 20 15:
    01234567890123456789
000 ....................
001 ....................
002 .........@..........
003 ....................
004 ....................
005 ....................
006 ....................
007 ....................
008 ....................
009 ....................
010 ....................
011 ....................
012 .........$..........
013 ....................
014 ....................
Piece 4 2:
OOOO
.OOO
```

And the bot has to respond with the coordinates where it wants to place the piece:
```bash
3 5
```

The game_engine will validate the move and update the map accordingly before sending the updated map and the next piece to the next bot.
If the game_engine detects an invlaid move, the bot responsible will not be able to make any more moves while the other bot can continue to play until there are no more valid moves for either bot.
It a bot crashes or fails to respond in time, it loses the game.

## Running the game with Docker
Read the [instructions](filler_docker.md) to run the game_engine with Docker.
In the instructions, we provide commands that bind the local directories to the container directories.
This allows us to easily access the solution code, maps, and logs from the host machine.

## Running the visualizer
Read the [instructions](./filler_visualizer/README.md) for visualizer.
Using the log files generated from the game engine, the visualizer can replay the game step by step.

## Commands for audits
```
./m1_game_engine -f maps/map00 -p1 solution/my_robot/target/release/my_robot -p2 m1_robots/wall_e > /filler/logs/game_log.txt 2>&1
./m1_game_engine -f maps/map00 -p2 solution/my_robot/target/release/my_robot -p1 m1_robots/wall_e > /filler/logs/game_log.txt 2>&1

./m1_game_engine -f maps/map01 -p1 solution/my_robot/target/release/my_robot -p2 m1_robots/h2_d2 > /filler/logs/game_log.txt 2>&1
./m1_game_engine -f maps/map01 -p2 solution/my_robot/target/release/my_robot -p1 m1_robots/h2_d2 > /filler/logs/game_log.txt 2>&1

./m1_game_engine -f maps/map02 -p1 solution/my_robot/target/release/my_robot -p2 m1_robots/bender > /filler/logs/game_log.txt 2>&1
./m1_game_engine -f maps/map02 -p2 solution/my_robot/target/release/my_robot -p1 m1_robots/bender > /filler/logs/game_log.txt 2>&1

./m1_game_engine -f maps/map02 -p1 solution/my_robot/target/release/my_robot -p2 m1_robots/terminator > /filler/logs/game_log.txt 2>&1
./m1_game_engine -f maps/map02 -p2 solution/my_robot/target/release/my_robot -p1 m1_robots/terminator > /filler/logs/game_log.txt 2>&1
```
## Collaborators 
- Allen [@AllenLeeyn](https://github.com/AllenLeeyn)
- Roope [@RuBoMa](https://github.com/RuBoMa)
- Johannes [@JSundb](https://github.com/JSundb)
