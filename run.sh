#!/usr/bin/env bash

# Step 1: Get the container ID of the running PostgreSQL container
CONTAINER_ID=$(docker ps --filter "ancestor=postgres:14" --format "{{.ID}}")

# Step 2: Remove the PostgreSQL container forcefully
if [ ! -z "$CONTAINER_ID" ]; then
    echo "Removing PostgreSQL container with ID: $CONTAINER_ID"
    docker rm -f $CONTAINER_ID
else
    echo "No PostgreSQL container found. Skipping removal."
fi

# Step 3: Kill the 'server' process
pkill server
echo "Server process killed."

# Step 3.5: reset the db if we changed the tables at all
sqlx db reset 

# Step 4: Run the cargo shuttle command
cargo shuttle run
