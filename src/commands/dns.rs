use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use hickory_resolver::AsyncResolver;
use std::time::Duration;
use tokio::time::Instant;

pub async fn check(domain: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing DNS resolution for {}...", domain);

    let mut opts = ResolverOpts::default();
    opts.timeout = Duration::from_secs(5);
    opts.attempts = 2;

    // Create resolver with hickory's AsyncResolver
    let resolver = AsyncResolver::tokio(ResolverConfig::default(), opts.clone());

    let iterations = 3;
    let mut total_time = Duration::default();
    let mut successful = 0;

    for i in 1..=iterations {
        println!("DNS lookup attempt {} of {}...", i, iterations);
        let start = Instant::now();

        match resolver.lookup_ip(domain).await {
            Ok(response) => {
                let duration = start.elapsed();
                total_time += duration;
                successful += 1;

                println!("Resolved in {} ms. Found addresses:", duration.as_millis());
                for addr in response.iter() {
                    println!("  - {}", addr);
                }
            }
            Err(e) => {
                println!("Lookup failed: {}", e);
            }
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    if successful > 0 {
        let avg_time = total_time.div_f64(successful as f64);
        println!("\nDNS Resolution Summary:");
        println!("Average resolution time: {} ms", avg_time.as_millis());
        println!("Successful lookups: {}/{}", successful, iterations);
    } else {
        println!(
            "\nAll DNS lookups failed. Please check your network connection and DNS settings."
        );
    }

    Ok(())
}
