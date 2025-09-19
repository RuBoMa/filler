#!/bin/bash

# Quick script to run a many test matches against a given opponent
# Usage: ./many_matches.sh [mac|win] [matches] [map] [opponent]
# Example: ./many_matches.sh mac 1000 map02 bender

set -e  # Exit on any error

# Default values
PLATFORM=""
MATCHES="100"
MAP="map00"
OPPONENT="terminator"

# Parse arguments
if [ $# -ge 1 ]; then
    PLATFORM=$1
fi
if [ $# -ge 2 ]; then
    MATCHES=$2
fi
if [ $# -ge 3 ]; then
    MAP=$3
fi
if [ $# -ge 4 ]; then
    OPPONENT=$4
fi

# Validate platform argument
if [ "$PLATFORM" != "mac" ] && [ "$PLATFORM" != "win" ]; then
    echo "Usage: $0 [mac|win] [matches] [map] [opponent]"
    echo "Example: $0 win map01 terminator"
    echo "Example: $0 mac map02 bender"
    exit 1
fi

echo "Running $MATCHES test matches against $OPPONENT on $MAP"

# Determine which game engine and robot directory to use
if [ "$PLATFORM" = "mac" ]; then
    GAME_ENGINE="./m1_game_engine"
    ROBOT_DIR="m1_robots"
else
    GAME_ENGINE="./linux_game_engine"
    ROBOT_DIR="linux_robots"
fi

wins=0
losses=0

player_total_score=0
opponent_total_score=0

for i in $(seq 1 $MATCHES); do
    echo "Running match $i..."
    
    # Capture the game output
    game_output=$(./quick_run.sh $PLATFORM $MAP $OPPONENT 2>&1)
    game_exit_code=$?
    
    # Parse the results from the output
    # Look for lines like "Player1 won!" or "Player2 won!"
    if echo "$game_output" | grep -q "Player1 won!"; then
        wins=$((wins + 1))
        # Extract Player1 score (our bot)
        player_score=$(echo "$game_output" | grep "Player1.*:" | sed 's/.*Player1.*: \([0-9]*\).*/\1/')
        if [ -n "$player_score" ] && [ "$player_score" -gt 0 ]; then
            player_total_score=$((player_total_score + player_score))
        else
            player_total_score=$((player_total_score + 1))
        fi
    elif echo "$game_output" | grep -q "Player2 won!"; then
        losses=$((losses + 1))
        # Extract Player2 score (opponent)
        opponent_score=$(echo "$game_output" | grep "Player2.*:" | sed 's/.*Player2.*: \([0-9]*\).*/\1/')
        if [ -n "$opponent_score" ] && [ "$opponent_score" -gt 0 ]; then
            opponent_total_score=$((opponent_total_score + opponent_score))
        else
            opponent_total_score=$((opponent_total_score + 1))
        fi
    else
        # If we can't determine winner, count as loss
        losses=$((losses + 1))
        opponent_total_score=$((opponent_total_score + 1))
    fi
done

echo "--------------------------------"
echo "Player wins: $wins"
echo "Player losses: $losses"
echo "--------------------------------"
echo "Player total score: $player_total_score"
echo "Opponent total score: $opponent_total_score"
echo "--------------------------------"
echo "Player win percentage: $((wins * 100 / $MATCHES))%"
echo "Opponent win percentage: $((losses * 100 / $MATCHES))%"
echo "--------------------------------"
echo "Player average score: $((player_total_score / $MATCHES))"
echo "Opponent average score: $((opponent_total_score / $MATCHES))"
echo "--------------------------------"