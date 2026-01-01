use bforge::git;

fn main() {
    let cache_dir = git::get_cache_dir().expect("Could not get cache directory");
    git::ensure_repo_cached("amirmo76/ui-vue", &cache_dir).expect("Could not cache repository");
}
