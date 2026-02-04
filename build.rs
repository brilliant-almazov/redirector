use std::process::Command;

fn main() {
    // Priority: GIT_COMMIT_SHORT env var (set via Docker build arg) > git command > "unknown"
    let commit = std::env::var("GIT_COMMIT_SHORT")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| {
            Command::new("git")
                .args(["rev-parse", "--short", "HEAD"])
                .output()
                .ok()
                .filter(|o| o.status.success())
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|| "unknown".to_string())
        });

    println!("cargo:rustc-env=GIT_COMMIT_SHORT={commit}");
    println!("cargo:rerun-if-env-changed=GIT_COMMIT_SHORT");

    // Track git ref changes for local builds
    println!("cargo:rerun-if-changed=.git/HEAD");
    if let Ok(head) = std::fs::read_to_string(".git/HEAD") {
        if let Some(ref_path) = head.trim().strip_prefix("ref: ") {
            println!("cargo:rerun-if-changed=.git/{ref_path}");
        }
    }
}
