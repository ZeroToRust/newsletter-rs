-- Add migration script here
-- create subscription table if it does not exits
--  with timestamp to keep track of when a user logged in 
DROP TABLE IF EXISTS subscriptions;

CREATE TABLE IF NOT EXISTS subscriptions (
        id UUID PRIMARY KEY,
        name TEXT NOT NULL,
        email TEXT NOT NULL UNIQUE,
        subscribed_at TIMESTAMPTZ NOT NULL
    );