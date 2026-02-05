CREATE TABLE IF NOT EXISTS geo_locations (
    id BIGSERIAL PRIMARY KEY,
    country_code CHAR(2) NOT NULL,
    city VARCHAR(255) NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_geo_locations_country_city
    ON geo_locations (country_code, city);

-- Reserved row: unknown location (id=1)
INSERT INTO geo_locations (country_code, city) VALUES ('--', '(unknown)')
    ON CONFLICT (country_code, city) DO NOTHING;

COMMENT ON TABLE geo_locations IS 'Reference table for GeoIP locations. id=1 is reserved for unknown/missing values';
COMMENT ON COLUMN geo_locations.id IS 'Auto-increment primary key';
COMMENT ON COLUMN geo_locations.country_code IS 'ISO 3166-1 alpha-2 country code. -- for unknown';
COMMENT ON COLUMN geo_locations.city IS 'City name in English from GeoIP database. (unknown) for missing';
