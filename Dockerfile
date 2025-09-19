FROM rust:1.75

RUN apt-get update && apt-get install -y \
    libsdl2-dev \
    libsdl2-ttf-dev \
    libgl1-mesa-dev \
    && rm -rf /var/lib/apt/lists/*

COPY ./maps			        /filler/maps
COPY ./linux_robots		    /filler/linux_robots
COPY ./m1_robots		    /filler/m1_robots
COPY ./linux_game_engine	/filler/linux_game_engine
COPY ./m1_game_engine	    /filler/m1_game_engine
COPY ./quick_run.sh		    /filler/quick_run.sh
COPY ./many_matches.sh	    /filler/many_matches.sh
COPY ./filler_visualizer    /filler/filler_visualizer

WORKDIR /filler/

# Make the quick run script executable
RUN chmod +x /filler/quick_run.sh
RUN chmod +x /filler/many_matches.sh

ENTRYPOINT /bin/bash
