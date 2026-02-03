# Load Testing

## Prerequisites

- [wrk](https://github.com/wg/wrk) - HTTP benchmarking tool
- Running redirector service
- Test data (hashids)

## Generate Test Data

If you have your own database:

```bash
# 1. Export IDs from your database
psql -d your_db -t -A -c "SELECT id FROM your_table ORDER BY RANDOM() LIMIT 10000;" > ids.txt

# 2. Generate hashids (requires redirector binary)
cargo build --release --bin gen_hashids
cat ids.txt | ./target/release/gen_hashids > hashids.txt
```

Or use the sample data provided:
- `sample_hashids_100.txt` - 100 hashids for cache hit testing
- `sample_hashids_10000.txt` - 10,000 hashids for mixed load

## Run Load Tests

### Test 1: 100% Cache Hit

```bash
# Warm up cache first
for h in $(cat tests/loadtest/sample_hashids_100.txt); do
    curl -s "http://localhost:8080/$h" > /dev/null
done

# Run load test
wrk -t4 -c100 -d30s -s tests/loadtest/loadtest.lua http://localhost:8080
```

### Test 2: Mixed Load (Cache Miss + DB)

```bash
# Clear cache
redis-cli FLUSHDB

# Run load test with larger dataset
HASHIDS_FILE=tests/loadtest/sample_hashids_10000.txt \
wrk -t4 -c100 -d30s -s tests/loadtest/loadtest.lua http://localhost:8080
```

## Expected Results

| Scenario | RPS | Avg Latency | P99 Latency |
|----------|-----|-------------|-------------|
| 100% Cache Hit | ~7,800 | ~14ms | ~50ms |
| Cache Miss (10K URLs) | ~2,300 | ~44ms | ~81ms |

**Test conditions**: wrk -t4 -c100 -d30s, macOS M1, PostgreSQL 15, Dragonfly (Redis)
