# Filler docker image

- To build the image `docker build -t filler .`
- To run the container `docker run -v "$(pwd)/solution":/filler/solution -it filler`. This instruction will open a terminal in the container, the directory `solution` will be mounted in the container as well.
- Example of a command in the container `./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator`
- Your solution should be inside the `solution` directory so it will be mounted and compiled inside the container and it will be able to be run in the game engine.

## Notes

- `Terminator` is a very strong robot so it's optional to beat him.
- For M1 Macs use `m1_robots` and `m1_game_engine`.

## Running the Filler Project ##

1. Build the docker image

```bash
docker build -t filler .
```

2. Run the Docker container

```bash
docker run -v "./solution":/filler/solution -it filler
```

3. Build your Rust bot

```bash
cd solution/my_robot
cargo build --release
```

4. Run a match

```bash
./m1_game_engine -f maps/map00 \
  -p1 solution/my_robot/target/release/my_robot \
  -p2 m1_robots/terminator
```
