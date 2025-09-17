FROM rust:1.75-buster

COPY ./maps			        /filler/maps
COPY ./linux_robots		    /filler/linux_robots
COPY ./m1_robots		    /filler/m1_robots
COPY ./linux_game_engine	/filler/linux_game_engine
COPY ./m1_game_engine	    /filler/m1_game_engine
COPY ./quick_run.sh		    /filler/quick_run.sh

WORKDIR /filler/

# Make the quick run script executable
RUN chmod +x /filler/quick_run.sh

ENTRYPOINT /bin/bash
