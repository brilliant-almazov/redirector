CREATE TABLE IF NOT EXISTS redirect_events (
    id BIGSERIAL,
    url_id BIGINT NOT NULL,
    event_timestamp TIMESTAMP NOT NULL,
    latency_micros BIGINT NOT NULL,
    source data_source NOT NULL,
    referer_id BIGINT NOT NULL,
    user_agent_id BIGINT NOT NULL,
    referer_domain_id BIGINT NOT NULL,
    geo_location_id BIGINT NOT NULL,
    ip TEXT,
    batch_id BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (id, event_timestamp)
) PARTITION BY RANGE (event_timestamp);

COMMENT ON TABLE redirect_events IS 'Redirect analytics events, partitioned by month';
COMMENT ON COLUMN redirect_events.id IS 'Auto-increment row ID (part of composite PK)';
COMMENT ON COLUMN redirect_events.url_id IS 'Decoded numeric URL ID from hashid';
COMMENT ON COLUMN redirect_events.event_timestamp IS 'When the redirect occurred (UTC), partition key';
COMMENT ON COLUMN redirect_events.latency_micros IS 'Request processing time in microseconds';
COMMENT ON COLUMN redirect_events.source IS 'Where URL was resolved from: cache or database';
COMMENT ON COLUMN redirect_events.referer_id IS 'FK to referers reference table. 1 = unknown/missing';
COMMENT ON COLUMN redirect_events.user_agent_id IS 'FK to user_agents reference table. 1 = unknown/missing';
COMMENT ON COLUMN redirect_events.referer_domain_id IS 'FK to referer_domains reference table. 1 = unknown/missing';
COMMENT ON COLUMN redirect_events.geo_location_id IS 'FK to geo_locations reference table. 1 = unknown/missing';
COMMENT ON COLUMN redirect_events.ip IS 'Client IP address (from X-Forwarded-For or X-Real-IP)';
COMMENT ON COLUMN redirect_events.batch_id IS 'Snowflake ID of the publisher batch';
COMMENT ON COLUMN redirect_events.created_at IS 'When the row was inserted into analytics DB (UTC)';

CREATE INDEX IF NOT EXISTS idx_redirect_events_url_ts
    ON redirect_events (url_id, event_timestamp);
