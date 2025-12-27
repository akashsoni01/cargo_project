#!/bin/bash

# Start Cassandra in the background
/docker-entrypoint.sh cassandra -f &

# Wait for Cassandra to be ready to accept CQL commands
until cqlsh -e "describe keyspaces"; do
    echo "Waiting for Cassandra to start..."
    sleep 1
done

# Run the CQL commands to set up the keyspace and table
cqlsh -f /scripts/init-keyspace.cql

# Keep the container running
tail -f /dev/null
