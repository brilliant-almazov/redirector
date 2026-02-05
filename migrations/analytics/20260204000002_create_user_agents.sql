CREATE TABLE IF NOT EXISTS user_agents (
    id BIGSERIAL PRIMARY KEY,
    hash CHAR(32) NOT NULL,
    value TEXT NOT NULL,
    browser VARCHAR(100),
    browser_version VARCHAR(50),
    os VARCHAR(100),
    device_type VARCHAR(20)
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_user_agents_hash ON user_agents (hash);

-- Reserved row: unknown user agent (id=1)
INSERT INTO user_agents (hash, value) VALUES ('00000000000000000000000000000000', '(unknown)')
    ON CONFLICT (hash) DO NOTHING;

COMMENT ON TABLE user_agents IS 'Reference table for HTTP User-Agent strings. id=1 is reserved for unknown/missing values';
COMMENT ON COLUMN user_agents.id IS 'Auto-increment primary key';
COMMENT ON COLUMN user_agents.hash IS 'MD5 hash of value, computed in application';
COMMENT ON COLUMN user_agents.value IS 'Full User-Agent string';
COMMENT ON COLUMN user_agents.browser IS 'Parsed browser name (e.g. Chrome, Firefox). NULL for sentinel row';
COMMENT ON COLUMN user_agents.browser_version IS 'Parsed browser version. NULL for sentinel row';
COMMENT ON COLUMN user_agents.os IS 'Parsed operating system (e.g. Windows, Linux, Android). NULL for sentinel row';
COMMENT ON COLUMN user_agents.device_type IS 'Device category: pc, smartphone, mobilephone, crawler, appliance, misc. NULL for sentinel row';
