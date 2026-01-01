# bforge 🛠️

> **🚧 Status: Work in Progress** This project is currently in active development. Command structures and configuration schemas are subject to change.

**bforge** is a universal CLI to scaffold and inject code patterns, components, or utilities from any git repository directly into your project.

Think of it as the "Copy Paste" architecture of [shadcn/ui](https://ui.shadcn.com/), but decoupled from any specific framework or library. It allows you to build your own internal registries for Vue, React, Rust, or any other language, and inject them into your codebase with a single command.

## 🚀 Why bforge?

Traditional package managers (`npm`) are great for libraries you don't touch. But for UI components, starter templates, or scaffolded logic, you often want **ownership** of the code.

-   **Universal Protocol:** Works with any language or framework.
    
-   **Decentralized:** No central registry server. Just Git.
    
-   **Smart Resolution:** Handles internal dependencies (e.g., a `Button` needing a `utils` file) and external dependencies (e.g., auto-installing `radix-vue` via npm).
    
-   **Repo Agnostic:** Should work with GitHub, GitLab, and private Git repositories. Only works with GitHub for now! 🫠
    

## 📦 Installation



```bash
cargo install bforge
```
## ⚡ Usage

### 1. Initialize

Run this in your project root to create a `bforge.config.json`. This tells bforge where to map files (e.g., "Put components in `src/components/ui`").

```bash
bforge init
```

### 2. Add an Item

Fetch a component, pattern, or utility from a remote repository.

```bash
# Syntax: bforge add <source> <item>

# Add a button from a GitHub
bforge add corp/ui button
```

## 🔧 Configuration

### User Config (`bforge.config.json`)

This file lives in your project root. It maps generic keys from the registry to actual paths in your project.

JSON

```
{
  "aliases": {
    "components": "src/components/ui",
    "composables": "src/composables",
    "utils": "src/lib",
    "types": "src/types"
  }
}

```

## 🏗️ Creating a Registry

Any Git repository can be a bforge source. You just need to add a `bforge.json` manifest at the root.

**`bforge.json` Example:**

```json
{
  "name": "my-ui-library",
  "items": {
    "button": {
      "type": "component",
      "description": "Primary button component",
      "dependencies": ["radix-vue", "clsx"],
      "internal_dependencies": ["utils"],
      "files": [
        {
          "source": "src/components/Button.vue",
          "target": "components/Button.vue"
        }
      ]
    },
    "utils": {
      "type": "utility",
      "files": [
        { "source": "src/lib/utils.ts", "target": "utils/index.ts" }
      ]
    }
  }
}

```

-   **`source`**: Path inside the remote git repo.
    
-   **`target`**: Abstract path. `components/Button.vue` will resolve to `src/components/ui/Button.vue` based on the user's config alias for `components`.
    

## 🛣️ Roadmap

-   [x] Basic Git cloning & caching
    
-   [x] Manifest parsing (`bforge.json`)
    
-   [x] File injection & path resolution
    
-   [x] NPM dependency auto-installation
    
-   [ ] Interactive prompts (`dialoguer` integration)
    
-   [ ] `update` command to refresh installed components

-   [ ] Become repo agnostic!
    
-   [ ] Support for private repos via SSH agent
    

## 📄 License

This project is licensed under the [MIT License](https://www.google.com/search?q=LICENSE).
