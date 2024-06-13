#!/bin/bash

# Pull the latest Redis image
#docker-compose pull redis

# Start the Redis container using Docker Compose
docker-compose up -d

#run the server
cargo run --release

# Check if Docker Compose command was successful
if [ $? -eq 0 ]; then
    echo "Redis container started successfully"
else
    echo "Failed to start Redis container"
    exit 1
fi
