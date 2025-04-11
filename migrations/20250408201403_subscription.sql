-- Add migration script here

CREATE TABLE IF NOT EXISTS users{
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    subscribed_at TIMESTAMP WITH TIME DEFAULT now()
}