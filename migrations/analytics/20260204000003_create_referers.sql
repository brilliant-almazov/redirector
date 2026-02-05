CREATE TABLE IF NOT EXISTS referers (
    id BIGSERIAL PRIMARY KEY,
    hash CHAR(32) NOT NULL,
    value TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_referers_hash ON referers (hash);

-- Reserved row: unknown referer (id=1)
INSERT INTO referers (hash, value) VALUES ('00000000000000000000000000000000', '(unknown)')
    ON CONFLICT (hash) DO NOTHING;

COMMENT ON TABLE referers IS 'Reference table for HTTP Referer URLs. id=1 is reserved for unknown/missing values';
COMMENT ON COLUMN referers.id IS 'Auto-increment primary key';
COMMENT ON COLUMN referers.hash IS 'MD5 hash of value, computed in application';
COMMENT ON COLUMN referers.value IS 'Full Referer URL';
