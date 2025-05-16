-- Add migration script here
CREATE TABLE
    IF NOT EXISTS subscriptions (
        id UUID PRIMARY KEY,
        name TEXT NOT NULL,
        email TEXT NOT NULL UNIQUE,
        subscribed_at TIMESTAMPTZ NOT NULL
    );