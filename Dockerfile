# Ubuntu 18.04, for openssl
FROM ubuntu@sha256:5f4bdc3467537cbbe563e80db2c3ec95d548a9145d64453b06939c4592d67b6d

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

# select build image
FROM rust:1.34 as build

# create a new empty shell project
RUN USER=root cargo new --bin drastical
WORKDIR /drastical

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/drastical*
RUN cargo build --release

# our final base
FROM rust:1.34

# copy the build artifact from the build stage
COPY --from=build /drastical/target/release/drastical .

# expose the port
EXPOSE 80

# set the startup command to run your binary
CMD ["./drastical"]

