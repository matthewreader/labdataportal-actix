-- Add migration script here
-- Create Samples Table
CREATE TABLE samples (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    sample_name VARCHAR(255) NOT NULL,
    sample_source VARCHAR(255) NOT NULL,
    collection_dtm TIMESTAMPTZ NOT NULL,
    received_dtm TIMESTAMPTZ NOT NULL,
    last_updated TIMESTAMPTZ NOT NULL
);