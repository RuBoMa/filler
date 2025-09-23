# filler docker

The project is designed by the school to run in a Docker container where the game_engine will run the solution bot against other bots.

Here are the instructions to run the game_engine with Docker and the commands to run the game with specified maps, bots and settings.

## Setup
1. Make sure you have [Docker](https://www.docker.com/get-started) installed on your machine.
2. Clone the repository to your local machine.
3. Open a terminal and navigate to the root directory of the cloned repository.
4. Copy your solution bot code into the `solution` directory.
5. Build the Docker image:
   ```bash
   docker build -t filler .
   ```
6. Run the Docker container with the following command:

    On macOS/Linux:
    ```bash
    docker run -it \
      -v "$(pwd)/solution:/filler/solution" \
      -v "$(pwd)/logs:/filler/logs" \
      -v "$(pwd)/maps:/filler/maps" \
      filler
    ```

    On Windows:
    ```powershell
    docker run -it `
      -v "${PWD}\solution:/filler/solution" `
      -v "${PWD}\logs:/filler/logs" `
      -v "${PWD}\maps:/filler/maps" `
      filler
    ```
    This command mounts the `solution`, `logs`, and `maps` directories from your host machine to the container, allowing you to access your bot code, game logs, and maps easily.

7. Inside the container, navigate to the `solution` directory and build your Rust bot:
    ```bash
    cd solution/my_robot
    cargo build --release
    cd ../..
    ```

8. Run a match using the game engine with your bot and an opponent bot:
    ```bash
    ./linux_game_engine -f maps/map01 -p1 solution/my_robot/target/release/my_robot -p2 linux_robots/wall_e
    ```
    You can replace `map01` with any map file in the `maps` directory and `terminator` with any bot in the `linux_robots` directory.
    To output the game log to a text file, you can use:
    ```bash
    ./linux_game_engine -f maps/map01 -p1 solution/my_robot/target/release/my_robot -p2 linux_robots/wall_e > /filler/logs/game_log.txt 2>&1
    ```

    For M1 Macs, use `m1_game_engine` and `m1_robots` instead:
    ```bash
    ./m1_game_engine -f maps/map01 -p1 solution/my_robot/target/release/my_robot -p2 m1_robots/wall_e
    ./m1_game_engine -f maps/map01 -p1 solution/my_robot/target/release/my_robot -p2 m1_robots/wall_e > /filler/logs/game_log.txt 2>&1
    ```

### Options for game engine
- bots:
  - `bender`
  - `h2_d2`
  - `wall_e`
  - `terminator` **optional bot. Toughest bot.**

- maps:
  - `map00` (20x15)
  - `map01` (40x30)
  - `map02` (100x100)

### Note for Windows Users
- If you get "invalid reference format" error, make sure to use the full Windows path format with backslashes
- The `--rm` flag automatically removes the container when you exit
- Use `linux_game_engine` and `linux_robots` on Windows (not `m1_*` versions)
