-- Enable GIS
CREATE EXTENSION postgis;

-- Store the geoJSON converted from the FIT files.
CREATE TABLE ride_geometries (
    id SERIAL PRIMARY KEY,
    rides_id INT NOT NULL,
    geometry GEOMETRY(MULTIPOLYGON, 4326),
    CONSTRAINT fk_ride
    FOREIGN KEY (rides_id)
    REFERENCES rides (id)
    ON DELETE CASCADE
);
