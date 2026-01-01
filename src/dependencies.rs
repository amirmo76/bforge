use std::{fs, path::Path, process::Command};

use colored::*;
use serde_json::Value;

pub fn install_missing_dependencies(deps: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if deps.is_empty() {
        return Ok(());
    }

    // 1. Filter out dependencies that are already in package.json
    let missing_deps = get_missing_deps(deps)?;

    if missing_deps.is_empty() {
        return Ok(());
    }

    // 2. Detect Package Manager (pnpm > yarn > bun > npm)
    let (cmd, install_arg) = detect_package_manager();

    println!(
        "{} Installing missing dependencies: {}",
        "📦".yellow(),
        missing_deps.join(", ").dimmed()
    );

    // 3. Run the command: e.g. "pnpm add clsx tailwind-merge"
    let status = Command::new(cmd)
        .arg(install_arg)
        .args(&missing_deps)
        .status()?;

    if !status.success() {
        return Err(format!("Failed to install dependencies using {}", cmd).into());
    }

    Ok(())
}

fn get_missing_deps(deps: &[String]) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let package_json_path = Path::new("package.json");

    if !package_json_path.exists() {
        return Err("package.json not found in the current directory.".into());
    }

    let content = fs::read_to_string(package_json_path)?;
    let json: Value = serde_json::from_str(&content)?;

    let mut missing = Vec::new();

    // Check both dependencies and devDependencies
    for dep in deps {
        let in_deps = json["dependencies"].get(dep).is_some();
        let in_dev_deps = json["devDependencies"].get(dep).is_some();

        if !in_deps && !in_dev_deps {
            missing.push(dep.clone());
        }
    }

    Ok(missing)
}

fn detect_package_manager() -> (&'static str, &'static str) {
    if Path::new("pnpm-lock.yaml").exists() {
        ("pnpm", "add")
    } else if Path::new("yarn.lock").exists() {
        ("yarn", "add")
    } else if Path::new("bun.lockb").exists() {
        ("bun", "add")
    } else {
        ("npm", "install")
    }
}
