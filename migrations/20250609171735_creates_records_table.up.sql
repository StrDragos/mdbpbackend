CREATE SCHEMA IF NOT EXISTS medpass;

-- Add up migration script here
CREATE TABLE records (
    id UUID PRIMARY KEY,
    user_id varchar(100) NOT NULL,
    file_name varchar(250),
    record_type Text,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NULL,
    facility_name varchar(250),
    title varchar(250),
    subtitle varchar(250),
    tags Text[]
)