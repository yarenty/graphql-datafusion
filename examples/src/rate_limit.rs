use async_std::task;
use futures::stream::StreamExt;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tracing::{info, warn};

pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
    window: Duration,
    burst_limit: u32,
}

impl RateLimiter {
    pub fn new(window: Duration, limit: u32, burst_limit: u32) -> Self {
        let semaphore = Arc::new(Semaphore::new(limit as usize));
        Self {
            semaphore,
            window,
            burst_limit,
        }
    }

    pub async fn acquire(&self) -> Result<(), String> {
        let permit = self.semaphore.clone().acquire().await;
        
        // Check burst limit
        let now = task::block_on(task::sleep(Duration::from_millis(0)));
        let permits = self.semaphore.available_permits();
        
        if permits < self.burst_limit as usize {
            warn!("Burst limit exceeded");
            return Err("Burst limit exceeded".to_string());
        }

        // Check window limit
        let window_start = now - self.window;
        let window_count = self.semaphore.available_permits();
        
        if window_count < 0 {
            warn!("Window limit exceeded");
            return Err("Window limit exceeded".to_string());
        }

        permit.ok();
        Ok(())
    }

    pub async fn execute_with_limit<F, T>(&self, f: F) -> Result<T, String>
    where
        F: FnOnce() -> T,
    {
        self.acquire().await?;
        Ok(f())
    }

    pub async fn execute_with_retry<F, T>(&self, f: F, max_retries: u32) -> Result<T, String>
    where
        F: Fn() -> T + Clone,
        T: Clone,
    {
        let mut retries = 0;
        loop {
            match self.acquire().await {
                Ok(_) => return Ok(f()),
                Err(e) => {
                    if retries >= max_retries {
                        return Err(e);
                    }
                    retries += 1;
                    task::sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Create rate limiter with 100 requests per minute, 5 burst requests
    let rate_limiter = RateLimiter::new(
        Duration::from_secs(60),
        100,
        5,
    );

    // Example function to rate limit
    let example_fn = || {
        info!("Executing rate-limited function");
        // Simulate some work
        task::sleep(Duration::from_millis(100)).await;
        "Success".to_string()
    };

    // Execute with rate limiting
    let result = rate_limiter.execute_with_limit(example_fn).await;
    info!("Result: {}", result.unwrap());

    // Execute with retry
    let result = rate_limiter.execute_with_retry(example_fn, 3).await;
    info!("Result with retry: {}", result.unwrap());

    // Test burst limit
    let mut tasks = Vec::new();
    for _ in 0..10 {
        let rate_limiter = rate_limiter.clone();
        let task = tokio::spawn(async move {
            match rate_limiter.execute_with_limit(example_fn).await {
                Ok(r) => info!("Task succeeded: {}", r),
                Err(e) => warn!("Task failed: {}", e),
            }
        });
        tasks.push(task);
    }

    // Wait for all tasks
    for task in tasks {
        task.await?;
    }

    Ok(())
}
