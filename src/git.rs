use std::{
    fs,
    path::{Path, PathBuf},
};

use colored::Colorize;
use directories::ProjectDirs;
use git2::{Repository, ResetType};

/// Helper to get the cache directory for git repos
pub fn get_cache_dir() -> Result<PathBuf, git2::Error> {
    let project_dirs = ProjectDirs::from("com", "amirmo76", "bforge").ok_or(
        git2::Error::from_str("Could not determine project directory"),
    )?;
    Ok(project_dirs.cache_dir().join("git"))
}

/// Ensures the repo is cached locally. Returns the path to the cached repo.
pub fn ensure_repo_cached(repo: &str, cache_dir: &Path) -> Result<PathBuf, git2::Error> {
    let folder_name = sanitize_repo_name_to_folder(repo);
    let repo_path = cache_dir.join(folder_name);
    println!("Caching repo {} to {:?}", repo.dimmed(), repo_path);

    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).map_err(|e| git2::Error::from_str(&e.to_string()))?;
    }

    if repo_path.exists() {
        // Repo already cached
        let repo = Repository::open(&repo_path)?;
        update_repo(&repo)?;
        Ok(repo_path)
    } else {
        // Clone the repo
        let url = get_url(repo);
        Repository::clone(&url, &repo_path)?;
        Ok(repo_path)
    }
}

/// updates the given repository to match the remote state
fn update_repo(repo: &Repository) -> Result<(), git2::Error> {
    let mut remote = repo.find_remote("origin")?;

    // 1. Fetch all branches from origin
    remote.fetch(&["refs/heads/*:refs/heads/*"], None, None)?;

    // 2. Find the commit that "origin/HEAD" points to (usually main/master)
    let head_ref = remote.default_branch()?;
    let head_name = head_ref
        .as_str()
        .ok_or(git2::Error::from_str("Could not determine default branch"))?;

    // 3. Resolve that reference to a Commit object
    let object = repo.find_reference(head_name)?.peel_to_commit()?;

    // 4. Hard Reset the local directory to match remote
    repo.reset(object.as_object(), ResetType::Hard, None)?;

    Ok(())
}

/// Helper to get the git url for a given repo
fn get_url(repo: &str) -> String {
    format!("https://github.com/{repo}.git")
}

/// Helper to make a clean folder name from a URL
fn sanitize_repo_name_to_folder(url: &str) -> String {
    url.replace(':', "_").replace('/', "_").replace('.', "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_url() {
        assert_eq!(get_url("user/repo"), "git@github.com:user/repo.git");
    }

    #[test]
    fn test_sanitize_repo_name() {
        assert_eq!(sanitize_repo_name_to_folder("user/repo"), "user_repo");
    }
}
