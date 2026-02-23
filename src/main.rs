use serde::Serialize;
use std::process::Command;
use sysinfo::{System}; 
use tokio;

#[derive(Serialize)]
struct AuditReport {
    gpu_status: String,
    rogue_processes: Vec<String>,
    critical_files_secure: bool,
    load_average: Vec<f64>,
}

#[tokio::main]
async fn main() {
    let mut sys = System::new();
    sys.refresh_processes();
    // system load doesn't require a refresh in sysinfo, it's a global call
    let load = System::load_average();

    let report = AuditReport {
        gpu_status: check_gpu().await,
        rogue_processes: find_rogue_procs(&sys),
        critical_files_secure: check_integrity(),
        load_average: vec![load.one, load.five],
    };

    // output minified JSON for high-speed automated ingestion
    println!("{}", serde_json::to_string(&report).unwrap());
}

async fn check_gpu() -> String {
    let output = Command::new("nvidia-smi")
        .args(["--query-gpu=utilization.gpu,temperature.gpu", "--format=csv,noheader,nounits"])
        .output();
    
    match output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        _ => "OFFLINE_OR_NO_DRIVER".to_string(),
    }
}

fn find_rogue_procs(sys: &System) -> Vec<String> {
    // Targeted scan for high-risk binaries often used in exfiltration or cryptojacking
    let blacklist = ["xmrig", "gdb", "strace", "tcpdump", "nc", "ncat"];
    sys.processes()
        .values()
        .filter(|p| blacklist.contains(&p.name()))
        .map(|p| format!("{}:{}", p.name(), p.pid()))
        .collect()
}

fn check_integrity() -> bool {
    // checks world-readable sensitive files, a common misconfiguration in rapid deployments
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata("/etc/shadow")
        .map(|m| (m.permissions().mode() & 0o007) == 0)
        .unwrap_or(true) // assume secure if file is missing (unlikely on Linux)
}

