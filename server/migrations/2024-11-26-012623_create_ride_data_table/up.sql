-- Store data files that are uploaded
-- by the user.
CREATE TABLE ride_data (
    id SERIAL PRIMARY KEY,
    created_date timestamp with time zone not NULL DEFAULT NOW(),
    rides_id INTEGER NOT NULL REFERENCES rides(id),
    description TEXT NOT NULL DEFAULT '',
    file_name VARCHAR(255) NOT NULL,
    file_type VARCHAR(8) NOT NULL
)
