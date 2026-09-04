fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Embed VERSION at compile time.
    let version = std::env::var("VERSION").unwrap_or_else(|_| "dev".to_string());
    println!("cargo:rustc-env=APP_VERSION={version}");
    println!("cargo:rerun-if-env-changed=VERSION");

    // Generate EMOJI_POOL constant from admin/emoji.json for device registration confirmation.
    println!("cargo:rerun-if-changed=admin/emoji.json");
    let json_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("admin/emoji.json");
    let data = std::fs::read_to_string(&json_path)?;
    let entries: Vec<serde_json::Value> = serde_json::from_str(&data)?;
    let pool: Vec<String> = entries
        .iter()
        .filter_map(|v| v["e"].as_str())
        .map(|s| format!("    {:?}", s))
        .collect();
    let code = format!(
        "pub const EMOJI_POOL: &[&str] = &[\n{}\n];\n",
        pool.join(",\n")
    );
    let out = std::env::var("OUT_DIR")?;
    std::fs::write(std::path::Path::new(&out).join("emoji_pool.rs"), code)?;
    Ok(())
}
