use colored::*;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

use crate::{
    dependencies, git,
    manifest::{self, FileMapping, Manifest, UserConfig},
};

pub fn handle_init() {
    let config_path = Path::new("bforge.config.json");
    if config_path.exists() {
        println!("{}", "bforge.config.json already exists.".yellow());
        return;
    }

    // Default configuration for a Vue project
    let default_config = UserConfig {
        aliases: HashMap::from([
            ("components".to_string(), "app/components/ui".to_string()),
            ("composables".to_string(), "app/composables".to_string()),
            ("utils".to_string(), "app/utils".to_string()),
        ]),
    };

    let json = serde_json::to_string_pretty(&default_config).expect("Failed to serialize config");
    fs::write(config_path, json).expect("Failed to write config");
    println!("{}", "Initialized bforge.config.json".green());
}

pub fn handle_add(repo_input: &str, item_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load User Config
    let config_path = Path::new("bforge.config.json");
    if !config_path.exists() {
        return Err("No bforge.config.json found. Run 'bforge init' first.".into());
    }
    let config_content = fs::read_to_string(config_path)?;
    let user_config: UserConfig = serde_json::from_str(&config_content)?;

    println!("{} Fetching {}...", "->".blue(), repo_input);

    // Cache dir
    let cache_dir = git::get_cache_dir()?;

    // 2. Cache Repo (using your git module)
    let repo_path = git::ensure_repo_cached(&repo_input, &cache_dir)?;

    // 3. Load Manifest
    let manifest = manifest::load_manifest(&repo_path)?;

    // 4. Install Item (Recursive)
    let mut installed = HashSet::new();
    install_item(
        item_name,
        &manifest,
        &repo_path,
        &user_config,
        &mut installed,
    )?;

    println!("\n{}", "Done!".green().bold());
    Ok(())
}

fn install_item(
    item_name: &str,
    manifest: &Manifest,
    repo_root: &Path,
    user_config: &UserConfig,
    installed: &mut HashSet<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if installed.contains(item_name) {
        return Ok(());
    }

    // 1. Find Item
    let item = manifest
        .items
        .get(item_name)
        .ok_or_else(|| format!("Item '{}' not found in manifest.", item_name))?;

    println!("{} Installing {}...", "+".green(), item_name);

    // 2. Handle Dependencies FIRST (Recursion)
    // External (NPM)
    if !item.dependencies.is_empty() {
        println!("  Dependencies: {:?}", item.dependencies);
        dependencies::install_missing_dependencies(&item.dependencies)?;
    }

    // Internal (Other items in the manifest)
    for dep in item.internal_dependencies.iter() {
        install_item(dep, manifest, repo_root, user_config, installed)?;
    }

    // 3. Copy Files
    for file in &item.files {
        copy_file(file, repo_root, user_config)?;
    }

    installed.insert(item_name.to_string());
    Ok(())
}

fn copy_file(
    mapping: &FileMapping,
    repo_root: &Path,
    user_config: &UserConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let source_path = repo_root.join(&mapping.source);
    let target_path = manifest::resolve_target_path(mapping, user_config);

    if !source_path.exists() {
        return Err(format!("File missing in registry repo: {:?}", source_path).into());
    }

    // Ensure parent dir exists
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(&source_path, &target_path)?;
    println!("  Created: {}", target_path.display().to_string().dimmed());

    Ok(())
}
