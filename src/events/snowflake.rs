use std::time::{Duration, SystemTime};

/// Custom epoch: 2025-01-01 00:00:00 UTC.
/// Extends i64 snowflake range from ~2039 (UNIX epoch) to ~2094.
const CUSTOM_EPOCH_MS: u64 = 1_735_689_600_000;

pub type SnowflakeGenerator = snowflaked::sync::Generator;

/// Create a thread-safe Snowflake ID generator with custom epoch.
///
/// Layout for i64 (63 usable bits):
/// - 41 bits: timestamp in ms since 2025-01-01 (~69 years)
/// - 10 bits: machine_id (0-1023)
/// - 12 bits: sequence (0-4095)
pub fn create_generator(machine_id: u16) -> SnowflakeGenerator {
    snowflaked::Builder::new()
        .instance(machine_id)
        .epoch(SystemTime::UNIX_EPOCH + Duration::from_millis(CUSTOM_EPOCH_MS))
        .build()
}

#[cfg(test)]
#[path = "snowflake_test.rs"]
mod snowflake_test;
