FROM ubuntu@sha256:5f4bdc3467537cbbe563e80db2c3ec95d548a9145d64453b06939c4592d67b6d

# select build image
FROM rust:1.34 as build

WORKDIR /drastical
COPY . .

RUN cargo build --release

# Ubuntu 18.04, for openssl
FROM ubuntu@sha256:5f4bdc3467537cbbe563e80db2c3ec95d548a9145d64453b06939c4592d67b6d
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /drastical/target/release/drastical /usr/local/bin/drastical

# expose the port
EXPOSE 80

WORKDIR /usr/local/bin
CMD ["./drastical"]
