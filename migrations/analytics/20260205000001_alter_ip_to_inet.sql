-- Change ip column from TEXT to INET for proper IP address storage.
-- PostgreSQL INET is 4 bytes (IPv4) or 16 bytes (IPv6) vs variable-length TEXT.
-- Supports network operators: <<, >>, &&, subnet containment checks.
ALTER TABLE redirect_events
    ALTER COLUMN ip TYPE INET USING ip::INET;
