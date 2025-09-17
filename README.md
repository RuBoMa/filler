# Filler docker image

- To build the image `docker build -t filler .`
- To run the container `docker run -v "$(pwd)/solution":/filler/solution -it filler`. This instruction will open a terminal in the container, the directory `solution` will be mounted in the container as well.
- Example of a command in the container `./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator`
- Your solution should be inside the `solution` directory so it will be mounted and compiled inside the container and it will be able to be run in the game engine.

## Notes

- `Terminator` is a very strong robot so it's optional to beat him.
- For M1 Macs use `m1_robots` and `m1_game_engine`.

## Running the Filler Project ##

### macOS/Linux Setup

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

### Windows Setup

1. Build the docker image

```powershell
docker build -t filler .
```

2. Run the Docker container

```powershell
docker run --rm -v C:\path\to\filler\solution:/filler/solution -it filler
```

**Note:** Replace `C:\path\to\filler` with your actual project path.

3. Build your Rust bot

```bash
cd /filler/solution/my_robot
cargo build --release
```

Then return to root for the next command:
```bash
cd ..
cd ..
```

4. Run a match

```bash
./linux_game_engine -f maps/map01 -p1 solution/my_robot/target/release/my_robot -p2 linux_robots/terminator
```

**Windows Troubleshooting:**
- If you get "invalid reference format" error, make sure to use the full Windows path format with backslashes
- The `--rm` flag automatically removes the container when you exit
- Use `linux_game_engine` and `linux_robots` on Windows (not `m1_*` versions)

## Quick Run Script

For faster development, you can use the provided quick run script that automatically rebuilds your bot and starts a game:

```bash
# Run with Windows/Linux game engine
./quick_run.sh win map01 terminator

# Run with Mac game engine
./quick_run.sh mac map02 bender

# Just rebuild and run with defaults (Windows/Linux engine)
./quick_run.sh win
```

**Script Arguments:**
- `[platform]`: `win` for Windows/Linux game engine, `mac` for Mac game engine
- `[map]`: Map file to use (default: `map01`)
- `[opponent]`: Robot opponent (default: `terminator`)

**Available opponents:** `bender`, `h2_d2`, `terminator`, `wall_e`
**Available maps:** `map00`, `map01`, `map02`
