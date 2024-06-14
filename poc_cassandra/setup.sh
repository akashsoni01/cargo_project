#!/bin/bash

# Pull the latest images
docker-compose pull

# Start the containers using Docker Compose
docker-compose up -d

# Check if Docker Compose command was successful
if [ $? -eq 0 ]; then
    echo "Containers started successfully"
else
    echo "Failed to start containers"
    exit 1
fi

# Example of a command to run your server (adjust as needed)
 cargo run --release

# Check if the server command was successful (uncomment if using)
 if [ $? -eq 0 ]; then
     echo "Server started successfully"
 else
     echo "Failed to start server"
     exit 1
 fi
