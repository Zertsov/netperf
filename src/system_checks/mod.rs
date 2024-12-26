use std::process::Command as ProcessCommand;

pub fn check_traceroute_availability() -> bool {
    let mut cmd = if cfg!(windows) {
        ProcessCommand::new("tracert")
    } else if cfg!(target_os = "linux") {
        ProcessCommand::new("tracepath")
    } else {
        ProcessCommand::new("traceroute")
    };

    cmd.arg("--help").output().is_ok()
} 