#!/bin/bash

# Quick script to rebuild Rust bot and run a game
# Usage: ./quick_run.sh [mac|win] [map] [opponent]
# Example: ./quick_run.sh win map01 terminator

set -e  # Exit on any error

# Default values
PLATFORM=""
MAP="map01"
OPPONENT="terminator"

# Parse arguments
if [ $# -ge 1 ]; then
    PLATFORM=$1
fi
if [ $# -ge 2 ]; then
    MAP=$2
fi
if [ $# -ge 3 ]; then
    OPPONENT=$3
fi

# Validate platform argument
if [ "$PLATFORM" != "mac" ] && [ "$PLATFORM" != "win" ]; then
    echo "Usage: $0 [mac|win] [map] [opponent]"
    echo "Example: $0 win map01 terminator"
    echo "Example: $0 mac map02 bender"
    exit 1
fi

echo "Building Rust bot..."
cd /filler/solution/my_robot
cargo build --release

echo "Returning to game directory..."
cd /filler

echo "Starting game..."

# Determine which game engine and robot directory to use
if [ "$PLATFORM" = "mac" ]; then
    GAME_ENGINE="./m1_game_engine"
    ROBOT_DIR="m1_robots"
else
    GAME_ENGINE="./linux_game_engine"
    ROBOT_DIR="linux_robots"
fi

# Run the game
echo "Running: $GAME_ENGINE -f maps/$MAP -p1 solution/my_robot/target/release/my_robot -p2 $ROBOT_DIR/$OPPONENT"
$GAME_ENGINE -f maps/$MAP -p1 solution/my_robot/target/release/my_robot -p2 $ROBOT_DIR/$OPPONENT
