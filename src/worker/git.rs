use anyhow::Result;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn init_repo_if_needed(repo_path: &str) -> Result<()> {
    if !Path::new(repo_path).exists() {
        fs::create_dir_all(repo_path)?;
        Command::new("git")
            .arg("init")
            .current_dir(repo_path)
            .output()?;

        Command::new("git")
            .args(&["config", "user.name", "egghead-service"])
            .current_dir(repo_path)
            .output()?;

        Command::new("git")
            .args(&["config", "user.email", "service@egghead.local"])
            .current_dir(repo_path)
            .output()?;
    }
    Ok(())
}

pub fn commit_script(
    repo_path: &str,
    file_path: &str,
    content: &str,
    message: &str,
) -> Result<String> {
    fs::write(file_path, content)?;

    Command::new("git")
        .arg("add")
        .arg(Path::new(file_path).file_name().unwrap())
        .current_dir(repo_path)
        .output()?;

    Command::new("git")
        .args(&["commit", "-m", message])
        .current_dir(repo_path)
        .output()?;

    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .current_dir(repo_path)
        .output()?;

    let sha = String::from_utf8(output.stdout)?
        .trim()
        .to_string();

    Ok(sha)
}
