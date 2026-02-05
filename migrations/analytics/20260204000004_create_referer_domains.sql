CREATE TABLE IF NOT EXISTS referer_domains (
    id BIGSERIAL PRIMARY KEY,
    domain VARCHAR(255) NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_referer_domains_domain ON referer_domains (domain);

-- Reserved row: unknown domain (id=1)
INSERT INTO referer_domains (domain) VALUES ('(unknown)')
    ON CONFLICT (domain) DO NOTHING;

COMMENT ON TABLE referer_domains IS 'Reference table for normalized referer domains. id=1 is reserved for unknown/missing values';
COMMENT ON COLUMN referer_domains.id IS 'Auto-increment primary key';
COMMENT ON COLUMN referer_domains.domain IS 'Normalized domain: lowercase, www. stripped, no trailing dot';
