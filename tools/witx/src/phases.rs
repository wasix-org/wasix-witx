use anyhow::{bail, Result};
use std::env;
use std::path::{Path, PathBuf};

pub fn docs_path(phase_paths: &[PathBuf]) -> PathBuf {
    phase_paths
        .get(0)
        .expect("at least one path")
        .parent()
        .expect("drop file")
        .join("../docs.md")
}

pub fn snapshot() -> Result<Vec<PathBuf>> {
    let root = repo_root()?;
    let snapshot = root.join("phases/snapshot/witx");
    let paths = vec![snapshot.join("wasix_v1.witx")];
    ensure_exists(&paths)?;
    Ok(paths)
}

pub fn ephemeral() -> Result<Vec<PathBuf>> {
    let root = repo_root()?;
    let ephemeral = root.join("phases/ephemeral/witx");
    let paths = vec![
        ephemeral.join("wasix_ephemeral_args.witx"),
        ephemeral.join("wasix_ephemeral_bus.witx"),
        ephemeral.join("wasix_ephemeral_clock.witx"),
        ephemeral.join("wasix_ephemeral_environ.witx"),
        ephemeral.join("wasix_ephemeral_fd.witx"),
        ephemeral.join("wasix_ephemeral_os.witx"),
        ephemeral.join("wasix_ephemeral_path.witx"),
        ephemeral.join("wasix_ephemeral_poll.witx"),
        ephemeral.join("wasix_ephemeral_proc.witx"),
        ephemeral.join("wasix_ephemeral_random.witx"),
        ephemeral.join("wasix_ephemeral_sched.witx"),
        ephemeral.join("wasix_ephemeral_sock.witx"),
        ephemeral.join("wasix_ephemeral_thread.witx"),
        ephemeral.join("wasix_ephemeral_tty.witx"),
    ];
    ensure_exists(&paths)?;
    Ok(paths)
}

pub mod old {
    use super::*;
    pub fn snapshot_0() -> Result<Vec<PathBuf>> {
        let root = repo_root()?;
        let snapshot_0 = root.join("phases/old/snapshot_0/witx");
        let paths = vec![snapshot_0.join("wasi_unstable.witx")];
        ensure_exists(&paths)?;
        Ok(paths)
    }
}

fn repo_root() -> Result<PathBuf> {
    let repo_root = if let Ok(e) = env::var("WASI_REPO") {
        PathBuf::from(e)
    } else {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
    };
    if repo_root.exists() {
        Ok(repo_root)
    } else {
        bail!("could not find WASI repo root - try setting WASI_REPO env variable")
    }
}

fn ensure_exists(paths: &[PathBuf]) -> Result<()> {
    for p in paths.iter() {
        if !p.exists() {
            bail!(
                "{} does not exist - is WASI_REPO set to repository root?",
                Path::display(p)
            )
        }
    }
    Ok(())
}
