use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use once_cell::sync::Lazy;
use crate::db::DbPool;
use crate::security::repository::SecurityRepository;

// Configuration
const MAX_ATTEMPTS: u32 = 5;
const LOCKOUT_DURATION_MINUTES: i64 = 15;
const ATTEMPT_WINDOW_SECONDS: u64 = 300; // 5 minutes to accumulate failures

// In-memory store: Map<IP, (Count, FirstAttemptTime)>
static FAILED_ATTEMPTS: Lazy<Arc<Mutex<HashMap<String, (u32, Instant)>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

pub struct BruteForceProtection;

impl BruteForceProtection {
    pub async fn check_and_record_failure(pool: &DbPool, ip: String) -> Result<(), anyhow::Error> {
        let mut attempts = FAILED_ATTEMPTS.lock().unwrap();
        
        // Clean up expired entries (naive implementation)
        // In production, use a proper cache with TTL like Redis
        
        let (count, first_attempt) = attempts.entry(ip.clone())
            .or_insert((0, Instant::now()));

        // Reset if window passed
        if first_attempt.elapsed() > Duration::from_secs(ATTEMPT_WINDOW_SECONDS) {
            *count = 0;
            *first_attempt = Instant::now();
        }

        *count += 1;

        if *count >= MAX_ATTEMPTS {
            // Ban the IP
            tracing::warn!("Brute force detected from IP: {}. Banning for {} minutes.", ip, LOCKOUT_DURATION_MINUTES);
            
            SecurityRepository::ban_entity(
                pool,
                "IP",
                &ip,
                Some("Brute Force Protection: Too many failed login attempts"),
                Some(LOCKOUT_DURATION_MINUTES),
                Some("SYSTEM"),
            ).await?;

            // Reset counter after ban
            attempts.remove(&ip);
            
            return Err(anyhow::anyhow!("Too many failed attempts. You have been temporarily banned."));
        }

        Ok(())
    }

    pub fn clear_attempts(ip: &str) {
        let mut attempts = FAILED_ATTEMPTS.lock().unwrap();
        attempts.remove(ip);
    }
}
