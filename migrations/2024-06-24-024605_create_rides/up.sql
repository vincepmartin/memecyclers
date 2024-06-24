-- Create the rides table
CREATE TABLE rides (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL DEFAULT ''
  );
