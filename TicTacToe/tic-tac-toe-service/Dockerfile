# ********************************************
#
# Tic-Tac-Toe Service
#
# Provides 2-client game-play of Tic-Tac-Toe.
#
# © 2024 Rust Made Easy. All rights reserved.
# @author JoelDavisEngineering@Gmail.com
#
# ********************************************

# This docker file provides 3-layers to take full advantage of docker image caching. If, for instance, the source code
# has not changed, a previously built latest_source image can be utilized.

# NOTE: Because Docker does not support ADD of files outside of the docker file context, and, because this service has
# code dependencies above its folder, it is necessary to invoke the docker build command from the course-work folder
# as follows:
#
#   cd [path to course-work]
#   docker build -t tic-tac-toe-service:latest -f ./TicTacToe/tic-tac-toe-service/Dockerfile .

# *** Source Code Layer ***
FROM scratch AS latest_source

ADD RustLibs/mqtt-publisher/src                /source/RustLibs/mqtt-publisher/src/
ADD RustLibs/mqtt-publisher/Cargo.toml         /source/RustLibs/mqtt-publisher/
ADD TicTacToe/tic-tac-toe-service/src/         /source/TicTacToe/tic-tac-toe-service/src/
ADD TicTacToe/tic-tac-toe-service/Cargo.toml   /source/TicTacToe/tic-tac-toe-service/

## *** Build Layer ***
FROM rust:1.81.0 AS builder

COPY --from=latest_source /source/   /source/

WORKDIR /source/TicTacToe/tic-tac-toe-service/

RUN cargo build --release

# *** Deployment Layer ***
FROM gcr.io/distroless/cc-debian12

# Copy just the service binaries ***
COPY --from=builder /source/TicTacToe/tic-tac-toe-service/target/release/*service   /usr/local/bin/

ENV RUST_LOG=info

EXPOSE 50020:50020

CMD ["tic-tac-toe-service"]