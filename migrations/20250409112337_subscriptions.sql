-- create subscription table if it does not exits
--  with timestamp to keep track of when a user logged in 
CREATE TABLE
    IF NOT EXISTS subscriptions (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name TEXT NOT NULL,
        email TEXT NOT NULL UNIQUE,
        subscribed_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT now ()
    );