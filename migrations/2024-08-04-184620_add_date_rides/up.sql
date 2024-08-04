-- Add a datetime column to the rides table
ALTER TABLE rides
ADD created_date timestamp with time zone not NULL DEFAULT NOW();
