-- Add migration script here
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(40), 
    password TEXT, 
    fullname TEXT, 
    created_at TIMESTAMP DEFAULT now()
);