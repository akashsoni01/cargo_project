CREATE KEYSPACE IF NOT EXISTS my_keyspace WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 };

USE my_keyspace;

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    name TEXT,
    email TEXT,
    password TEXT
);

INSERT INTO users (id, name, email, password) VALUES (uuid(), 'Alice', 'alice@example.com', 'password1');
INSERT INTO users (id, name, email, password) VALUES (uuid(), 'Bob', 'bob@example.com', 'password2');
INSERT INTO users (id, name, email, password) VALUES (uuid(), 'Charlie', 'charlie@example.com', 'password3');
