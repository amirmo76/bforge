use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub name: String,
    pub schema_version: String,
    pub items: HashMap<String, Item>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    #[serde(rename = "type")]
    pub item_type: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub internal_dependencies: Vec<String>,
    pub files: Vec<FileMapping>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileMapping {
    pub source: String,
    pub target: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfig {
    aliases: HashMap<String, String>,
}

// load manifest from a json file
pub fn load_manifest(path: &Path) -> Result<Manifest, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(path)?;
    let manifest = serde_json::from_str::<Manifest>(&file_content)?;
    Ok(manifest)
}

// load user config from a json file
pub fn load_user_config(path: &Path) -> Result<UserConfig, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(path)?;
    let config: UserConfig = serde_json::from_str(&file_content)?;
    Ok(config)
}

// resolve target path for a file mapping
pub fn resolve_target_path(mapping: &FileMapping, config: &UserConfig) -> PathBuf {
    let target = mapping.target.clone();
    let target_parts: Vec<&str> = target.splitn(2, '/').collect();

    // target does not have enough parts to contain an alias
    if target_parts.len() < 2 {
        return Path::new(mapping.source.as_str()).to_path_buf();
    }

    for (alias, path) in config.aliases.iter() {
        if alias == target_parts[0] {
            return Path::new(path).join(target_parts[1]);
        }
    }

    // alias not found, return source path
    Path::new(mapping.source.as_str()).to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_target_path_success() {
        let mut aliases = HashMap::new();
        aliases.insert("components".to_string(), "app/components".to_string());
        let config = UserConfig { aliases };

        let mapping = FileMapping {
            source: "src/app/components/button/Button.vue".to_string(),
            target: "components/ui/button/Button.vue".to_string(),
        };

        let result = resolve_target_path(&mapping, &config);
        assert_eq!(result, PathBuf::from("app/components/ui/button/Button.vue"));
    }

    #[test]
    fn test_resolve_target_path_unknown_alias() {
        let config = UserConfig {
            aliases: HashMap::new(),
        };
        let mapping = FileMapping {
            source: "local/file".to_string(),
            target: "unknown/file".to_string(),
        };

        let result = resolve_target_path(&mapping, &config);
        // Fallback to source path
        assert_eq!(result, PathBuf::from("local/file"));
    }

    #[test]
    fn test_resolve_target_path_no_slash() {
        let config = UserConfig {
            aliases: HashMap::new(),
        };
        let mapping = FileMapping {
            source: "local/file".to_string(),
            target: "simple_name".to_string(),
        };

        let result = resolve_target_path(&mapping, &config);
        // Not enough parts to split, fallback to source
        assert_eq!(result, PathBuf::from("local/file"));
    }

    #[test]
    fn test_load_manifest_file_not_found() {
        let res = load_manifest(Path::new("non_existent_file.json"));
        assert!(res.is_err());
    }

    #[test]
    fn load_manifest_valid_file() {
        let file_path = Path::new("test_manifest.json");
        let json_content = r#"{
    "name": "ui-vue",
    "schema_version": "1.0.0",
    "items": {
        "button": {
            "type": "component",
            "description": "Standard button component with variants.",
            "files": [
                {
                    "source": "app/components/button/Button.vue",
                    "target": "components/button/Button.vue"
                }
            ],
            "dependencies": ["vue", "class-variance-authority"],
            "internal_dependencies": []
        }
    }
}"#;
        fs::write(file_path, json_content).expect("could not write test manifest file");
        let res = load_manifest(&file_path);
        assert!(res.is_ok());
        let manifest = res.unwrap();
        assert_eq!(manifest.name, "ui-vue");
        assert_eq!(manifest.schema_version, "1.0.0");
        assert!(manifest.items.contains_key("button"));
        assert_eq!(manifest.items["button"].item_type, "component");
        let _ = fs::remove_file(file_path);
    }
}
