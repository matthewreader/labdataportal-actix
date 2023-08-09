-- Add migration script here
-- Create Samples Table
CREATE TABLE samples (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    sample_name VARCHAR(255),
    sample_source VARCHAR(255),
    collection_dtm TIMESTAMP,
    received_dtm TIMESTAMP,
    last_updated TIMESTAMP
);