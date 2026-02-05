//! Load testing tool for the redirector service.
//!
//! Fetches random URL IDs from the source database, encodes them as hashids,
//! and sends HTTP GET requests to the redirector at a controlled RPS.
//!
//! Usage:
//!   cargo run --release --bin load_test -- [OPTIONS]
//!
//! Options:
//!   --rps <N>       Requests per second (default: 100)
//!   --count <N>     Number of URLs to fetch (default: 10000)
//!   --target <URL>  Base URL (default: http://localhost:3000)
//!   --db-url <URL>  Source database (default: postgresql://postgres:postgres@localhost:5432/dev)
//!   --salt <SALT>   Hashid salt (default: test-salt-for-load-testing)
//!   --loop          Loop through URLs indefinitely until Ctrl+C

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use rand::Rng;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::Semaphore;

/// Well-known IP ranges from different countries for geo diversity.
/// Each entry: (base_ip_octets, country hint).
const GEO_IP_POOLS: &[([u8; 3], &str)] = &[
    ([8, 8, 8], "US"),       // Google DNS, US
    ([1, 1, 1], "AU"),       // Cloudflare, AU
    ([77, 88, 55], "RU"),    // Yandex, Russia
    ([139, 162, 130], "DE"), // Linode, Germany
    ([163, 177, 112], "BR"), // Brazil
    ([41, 206, 0], "NG"),    // Nigeria
    ([196, 216, 2], "ZA"),   // South Africa
    ([103, 21, 244], "IN"),  // India
    ([110, 232, 176], "JP"), // Japan
    ([175, 45, 176], "CN"),  // China
    ([91, 198, 174], "GB"),  // UK
    ([185, 70, 40], "NL"),   // Netherlands
    ([156, 146, 56], "CA"),  // Canada
    ([45, 55, 99], "SG"),    // Singapore, DigitalOcean
    ([200, 7, 4], "AR"),     // Argentina
    ([168, 119, 0], "FR"),   // France, OVH
    ([5, 101, 0], "SE"),     // Sweden
    ([31, 13, 64], "IE"),    // Ireland, Facebook
    ([203, 104, 0], "KR"),   // South Korea
    ([62, 149, 0], "TR"),    // Turkey
];

/// Generate a random IP from geo-diverse pools.
fn random_ip(rng: &mut impl Rng) -> String {
    let pool = &GEO_IP_POOLS[rng.gen_range(0..GEO_IP_POOLS.len())];
    let last_octet: u8 = rng.gen_range(1..=254);
    format!("{}.{}.{}.{}", pool.0[0], pool.0[1], pool.0[2], last_octet)
}

/// Random User-Agent strings for diversity.
const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (iPad; CPU OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:128.0) Gecko/20100101 Firefox/128.0",
    "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
    "curl/8.4.0",
    "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
];

const REFERER_DOMAINS: &[&str] = &[
    "https://www.google.com/search?q=test",
    "https://yandex.ru/search/?text=test",
    "https://t.co/abc123",
    "https://www.facebook.com/",
    "https://news.ycombinator.com/item?id=12345",
    "https://www.reddit.com/r/programming/",
    "https://t.me/channel/123",
    "", // no referer
    "",
    "", // ~30% without referer
];

fn parse_arg(args: &[String], flag: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == flag).map(|w| w[1].clone())
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
}

struct Stats {
    total: AtomicU64,
    success: AtomicU64,
    errors: AtomicU64,
    total_latency_us: AtomicU64,
}

impl Stats {
    fn new() -> Self {
        Self {
            total: AtomicU64::new(0),
            success: AtomicU64::new(0),
            errors: AtomicU64::new(0),
            total_latency_us: AtomicU64::new(0),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let rps: u64 = parse_arg(&args, "--rps")
        .and_then(|v| v.parse().ok())
        .unwrap_or(100);
    let count: i64 = parse_arg(&args, "--count")
        .and_then(|v| v.parse().ok())
        .unwrap_or(10_000);
    let target =
        parse_arg(&args, "--target").unwrap_or_else(|| "http://localhost:3000".to_string());
    let db_url = parse_arg(&args, "--db-url")
        .unwrap_or_else(|| "postgresql://postgres:postgres@localhost:5432/dev".to_string());
    let salt =
        parse_arg(&args, "--salt").unwrap_or_else(|| "test-salt-for-load-testing".to_string());
    let loop_mode = has_flag(&args, "--loop");

    eprintln!("=== Redirector Load Test ===");
    eprintln!("Target:    {}", target);
    eprintln!("RPS:       {}", rps);
    eprintln!("Count:     {}", count);
    eprintln!("DB:        {}", db_url);
    eprintln!("Salt:      {}", salt);
    eprintln!("Loop:      {}", loop_mode);
    eprintln!();

    // --- Step 1: Fetch random URL IDs ---
    eprintln!("Fetching {} random URLs from database...", count);
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await?;

    let rows: Vec<(i64,)> =
        sqlx::query_as("SELECT id FROM dictionary.urls ORDER BY RANDOM() LIMIT $1")
            .bind(count)
            .fetch_all(&pool)
            .await?;
    pool.close().await;

    let ids: Vec<i64> = rows.into_iter().map(|r| r.0).collect();
    eprintln!("Fetched {} URL IDs", ids.len());

    if ids.is_empty() {
        anyhow::bail!("No URLs found in database");
    }

    // --- Step 2: Encode hashids ---
    let harsh = harsh::Harsh::builder()
        .salt(salt)
        .length(6)
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to create hashid encoder: {:?}", e))?;

    let hashids: Vec<String> = ids.iter().map(|&id| harsh.encode(&[id as u64])).collect();
    eprintln!("Encoded {} hashids", hashids.len());
    eprintln!();

    // --- Step 3: Setup HTTP client ---
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(10))
        .pool_max_idle_per_host(rps.min(200) as usize)
        .build()?;

    // --- Step 4: Run load test ---
    let stats = Arc::new(Stats::new());
    let total_target = hashids.len() as u64;
    let start_time = Instant::now();

    // Graceful shutdown flag
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_signal = shutdown.clone();
    tokio::spawn(async move {
        let _ = tokio::signal::ctrl_c().await;
        shutdown_signal.store(true, Ordering::SeqCst);
        eprintln!("\nShutting down...");
    });

    // Spawn periodic reporter
    let stats_ref = stats.clone();
    let shutdown_reporter = shutdown.clone();
    let reporter = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        let mut prev_total = 0u64;
        loop {
            interval.tick().await;
            if shutdown_reporter.load(Ordering::Relaxed) {
                break;
            }
            let elapsed = start_time.elapsed().as_secs();
            let total = stats_ref.total.load(Ordering::Relaxed);
            let success = stats_ref.success.load(Ordering::Relaxed);
            let errors = stats_ref.errors.load(Ordering::Relaxed);
            let latency_us = stats_ref.total_latency_us.load(Ordering::Relaxed);
            let current_rps = total.saturating_sub(prev_total);
            prev_total = total;

            let avg_latency_ms = if total > 0 {
                latency_us as f64 / total as f64 / 1000.0
            } else {
                0.0
            };

            eprintln!(
                "[{:>4}s] {}/{} | {} rps | avg {:.1}ms | ok {} err {}",
                elapsed, total, total_target, current_rps, avg_latency_ms, success, errors,
            );
        }
    });

    // Rate-controlled request loop
    let concurrency = (rps * 2).max(10) as usize;
    let semaphore = Arc::new(Semaphore::new(concurrency));
    let nanos_per_request = 1_000_000_000u64 / rps.max(1);
    let mut interval = tokio::time::interval(Duration::from_nanos(nanos_per_request));

    let hashid_iter: Box<dyn Iterator<Item = &String>> = if loop_mode {
        Box::new(hashids.iter().cycle())
    } else {
        Box::new(hashids.iter())
    };

    let mut rng = rand::thread_rng();

    for hashid in hashid_iter {
        if shutdown.load(Ordering::Relaxed) {
            break;
        }
        interval.tick().await;

        let permit = semaphore.clone().acquire_owned().await?;
        let client = client.clone();
        let url = format!("{}/r/{}", target, hashid);
        let stats = stats.clone();

        // Generate random headers for geo/UA diversity
        let ip = random_ip(&mut rng);
        let ua = USER_AGENTS[rng.gen_range(0..USER_AGENTS.len())].to_string();
        let referer_val = REFERER_DOMAINS[rng.gen_range(0..REFERER_DOMAINS.len())].to_string();

        tokio::spawn(async move {
            let req_start = Instant::now();
            let mut req = client
                .get(&url)
                .header("X-Forwarded-For", &ip)
                .header("User-Agent", &ua);
            if !referer_val.is_empty() {
                req = req.header("Referer", &referer_val);
            }
            match req.send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        stats.success.fetch_add(1, Ordering::Relaxed);
                    } else {
                        stats.errors.fetch_add(1, Ordering::Relaxed);
                    }
                }
                Err(_) => {
                    stats.errors.fetch_add(1, Ordering::Relaxed);
                }
            }
            let latency = req_start.elapsed();
            stats
                .total_latency_us
                .fetch_add(latency.as_micros() as u64, Ordering::Relaxed);
            stats.total.fetch_add(1, Ordering::Relaxed);
            drop(permit);
        });
    }

    // Wait for in-flight requests
    eprintln!("\nWaiting for in-flight requests...");
    let _ = semaphore.acquire_many(concurrency as u32).await;

    shutdown.store(true, Ordering::SeqCst);
    reporter.abort();

    // --- Step 5: Print summary ---
    let elapsed = start_time.elapsed();
    let total = stats.total.load(Ordering::Relaxed);
    let success = stats.success.load(Ordering::Relaxed);
    let errors = stats.errors.load(Ordering::Relaxed);
    let latency_us = stats.total_latency_us.load(Ordering::Relaxed);

    let avg_latency_ms = if total > 0 {
        latency_us as f64 / total as f64 / 1000.0
    } else {
        0.0
    };
    let actual_rps = if elapsed.as_secs_f64() > 0.0 {
        total as f64 / elapsed.as_secs_f64()
    } else {
        0.0
    };

    eprintln!();
    eprintln!("=== LOAD TEST COMPLETE ===");
    eprintln!("Total requests:  {}", total);
    eprintln!("Duration:        {:.1}s", elapsed.as_secs_f64());
    eprintln!("Avg RPS:         {:.1}", actual_rps);
    eprintln!("Avg latency:     {:.1}ms", avg_latency_ms);
    eprintln!(
        "Success:         {} ({:.1}%)",
        success,
        if total > 0 {
            success as f64 / total as f64 * 100.0
        } else {
            0.0
        }
    );
    eprintln!(
        "Errors:          {} ({:.1}%)",
        errors,
        if total > 0 {
            errors as f64 / total as f64 * 100.0
        } else {
            0.0
        }
    );
    eprintln!("==========================");

    Ok(())
}
