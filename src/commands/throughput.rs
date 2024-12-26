use futures_util::StreamExt;
use reqwest::Client;
use tokio::time::Instant;

pub async fn check() -> Result<(), Box<dyn std::error::Error>> {
    let test_servers = [
        "https://speed.cloudflare.com/__down", // Cloudflare
        "https://download.microsoft.com/download/2/0/E/20E90413-712F-438C-988E-FDAA79A8AC3D/dotnetfx35.exe", // Microsoft
        "https://proof.ovh.net/files/100Mb.dat", // OVH
        "https://speed.hetzner.de/100MB.bin", // Hetzner
    ];

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    println!("Testing download throughput...");

    for test_url in test_servers {
        println!("Attempting download from {}...", test_url);

        match test_download(&client, test_url).await {
            Ok(_) => return Ok(()),
            Err(e) => println!("Failed to test with {}: {}", test_url, e),
        }
    }

    Err("All download tests failed".into())
}

async fn test_download(client: &Client, test_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let response = client.get(test_url).send().await?;
    let content_length = response.content_length().unwrap_or(0);

    if content_length == 0 {
        return Err("Failed to determine content length".into());
    }

    let mut stream = response.bytes_stream();
    let mut total_bytes = 0;
    let mut last_print = Instant::now();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        total_bytes += chunk.len();

        if last_print.elapsed().as_secs() >= 1 {
            let elapsed = start.elapsed().as_secs_f64();
            let mb_downloaded = total_bytes as f64 / 1_000_000.0;
            let current_speed = mb_downloaded / elapsed;

            println!(
                "Progress: {:.2} MB downloaded, Current Speed: {:.2} MB/s",
                mb_downloaded, current_speed
            );
            last_print = Instant::now();
        }
    }

    let elapsed = start.elapsed();
    let mb_downloaded = total_bytes as f64 / 1_000_000.0;
    let time_in_seconds = elapsed.as_secs_f64();
    let avg_speed = mb_downloaded / time_in_seconds;

    println!("\nDownload test completed:");
    println!("Total downloaded: {:.2} MB", mb_downloaded);
    println!("Time taken: {:.2} seconds", time_in_seconds);
    println!("Average speed: {:.2} MB/s", avg_speed);

    Ok(())
}
