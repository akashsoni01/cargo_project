# Stage 1: Build the Rust application
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/poc_redis_actix

# Copy all files from the current directory to the working directory
COPY . .

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create the final image with the built application
FROM ubuntu:latest

# Set the working directory
WORKDIR /usr/src/poc_redis_actix

# Copy the built application from the builder stage
COPY --from=builder /usr/src/poc_redis_actix/target/release/poc_redis_actix .

# Label information
LABEL authors="akashsoni"

# Set the entrypoint
ENTRYPOINT ["./poc_redis_actix"]

# Open a terminal, navigate to the directory containing your Dockerfile,
# and run the following command:
# docker build -t poc_redis_actix .
# Run the following command to start a container from the image:
# docker run --rm poc_redis_actix