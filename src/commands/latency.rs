use tokio::time::Instant;

pub async fn check(count: u32) -> Result<(), Box<dyn std::error::Error>> {
    let target = "8.8.8.8:53"; // Google DNS server on port 53

    println!("Testing latency to {} for {} attempts...", target, count);

    let mut total_latency = 0.0;
    let mut successful_pings = 0;

    for _ in 0..count {
        let start = Instant::now();
        match tokio::net::TcpStream::connect(target).await {
            Ok(_) => {
                let duration = start.elapsed();
                println!("Response time: {} ms", duration.as_millis());
                total_latency += duration.as_millis() as f64;
                successful_pings += 1;
            }
            Err(err) => {
                println!("Connection failed: {}", err);
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    if successful_pings > 0 {
        let avg_latency = total_latency / successful_pings as f64;
        println!(
            "Average latency: {:.2} ms ({} successful attempts)",
            avg_latency, successful_pings
        );
    } else {
        println!("All connection attempts failed. Check your network connection.");
    }

    Ok(())
}
