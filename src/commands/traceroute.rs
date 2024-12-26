use std::process::Command as ProcessCommand;

#[cfg(target_os = "windows")]
pub async fn perform(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Performing traceroute to {}...", host);
    
    let output = ProcessCommand::new("tracert").arg(host).output()?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        println!("{}", result);
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("Traceroute failed: {}", error);
    }

    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub async fn perform(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Performing traceroute to {}...", host);
    
    let traceroute_cmd = if cfg!(target_os = "linux") {
        "tracepath"
    } else {
        "traceroute"
    };

    let output = ProcessCommand::new(traceroute_cmd).arg(host).output()?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        println!("{}", result);
    } else {
        let alt_cmd = if traceroute_cmd == "tracepath" {
            "traceroute"
        } else {
            "tracepath"
        };
        let alt_output = ProcessCommand::new(alt_cmd).arg(host).output();

        match alt_output {
            Ok(output) if output.status.success() => {
                let result = String::from_utf8_lossy(&output.stdout);
                println!("{}", result);
            }
            _ => {
                let error = String::from_utf8_lossy(&output.stderr);
                eprintln!("Traceroute failed: {}", error);
            }
        }
    }

    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
pub async fn perform(host: &str) -> Result<(), Box<dyn std::error::Error>> {
    Err("Traceroute is not supported on this operating system".into())
} 