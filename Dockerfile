# Run detached and remove container when it stopped
# docker build -t pg-server .
# docker run -dp 8080:3030 --rm --name server pg-server

FROM rust:1.66 as build

# create a new empty shell project
RUN USER=root cargo new --bin pg-server
WORKDIR /pg-server

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/pg-server*
RUN cargo build --release

# our final slim base
FROM rust:1.66-slim-buster
# TODO: use even more slim image
#FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /pg-server/target/release/pg-server .

# set the startup command to run your binary
CMD ["./pg-server"]
