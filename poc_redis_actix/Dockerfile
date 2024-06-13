## Stage 1: Build the Rust application
#FROM rust:latest as builder
#
## Set the working directory
#WORKDIR /usr/src/poc_redis_actix
#
## Copy all files frsom the current directory to the working directory
#COPY . .
#
## Build the application in release mode
#RUN apt-get update && apt-get install -y libssl-dev pkg-config
#RUN cargo build --release
#
## Stage 2: Create the final image with the built application
#FROM ubuntu:latest
#
## Install necessary dependencies
#RUN apt-get update && apt-get install -y libssl-dev pkg-config
#
## Set the working directory
#WORKDIR /usr/src/poc_redis_actix
#
## Copy the built application from the builder stage
#COPY --from=builder /usr/src/poc_redis_actix/target/release/poc_redis_actix .
#
## Expose the application port
#EXPOSE 8080
#EXPOSE 6379
#
## Start the application
#CMD ./poc_redis_actix
