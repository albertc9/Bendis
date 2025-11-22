use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BendisConfig {
    /// Silent mode: suppress bender output during cache preparation
    /// 1 = on (default), 0 = off
    #[serde(default = "default_silent_mode")]
    pub silent_mode: u8,

    /// Storage saving mode: clean up bendis_workspace/.bender/ after update
    /// 0 = off (default, keep cache), 1 = on (delete entire bendis_workspace/.bender/)
    #[serde(default = "default_storage_saving_mode")]
    pub storage_saving_mode: u8,

    /// GitIgnore check: auto-manage bendis_workspace/.gitignore file
    /// 1 = on (default), 0 = off
    #[serde(default = "default_gitignore_check")]
    pub gitignore_check: u8,

    /// First run flag: whether this is the first time running bendis
    /// 0 = already shown welcome (default), 1 = need to show welcome
    #[serde(default = "default_first_run")]
    pub first_run: u8,

    /// Last run version: track the version to detect updates
    #[serde(default = "default_version")]
    pub version: String,
}

fn default_silent_mode() -> u8 {
    1
}

fn default_storage_saving_mode() -> u8 {
    0
}

fn default_gitignore_check() -> u8 {
    1
}

fn default_first_run() -> u8 {
    1
}

fn default_version() -> String {
    String::new()
}

impl Default for BendisConfig {
    fn default() -> Self {
        Self {
            silent_mode: default_silent_mode(),
            storage_saving_mode: default_storage_saving_mode(),
            gitignore_check: default_gitignore_check(),
            first_run: default_first_run(),
            version: default_version(),
        }
    }
}

impl BendisConfig {
    /// Load configuration from global config file
    /// If file doesn't exist, create it with default values
    pub fn load() -> Result<Self> {
        let config_path = get_config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .context("Failed to read bendis config.toml")?;
            let config: BendisConfig = toml::from_str(&content)
                .context("Failed to parse bendis config.toml")?;
            Ok(config)
        } else {
            // Create default config file
            let default_config = BendisConfig::default();
            default_config.save()?;
            Ok(default_config)
        }
    }

    /// Save configuration to global config file
    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path()?;

        // Ensure config directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create config directory")?;
        }

        // Add comments to the generated TOML
        let content_with_comments = format!(
"# Bendis Global Configuration File
#
# This file controls the behavior of bendis (Bender Integration System)
# Location: {}

# Silent mode: suppress bender output during cache preparation
# 1 = on (default), 0 = off
silent_mode = {}

# Storage saving mode: clean up bendis_workspace/.bender/ after update
# 0 = off (default, keep cache), 1 = on (delete entire bendis_workspace/.bender/)
storage_saving_mode = {}

# GitIgnore check: auto-manage bendis_workspace/.gitignore file
# 1 = on (default), 0 = off
gitignore_check = {}

# First run flag: whether this is the first time running bendis
# 0 = already shown welcome, 1 = need to show welcome (default)
first_run = {}

# Last run version: used to detect version updates
version = \"{}\"
",
            config_path.display(),
            self.silent_mode,
            self.storage_saving_mode,
            self.gitignore_check,
            self.first_run,
            self.version
        );

        fs::write(&config_path, content_with_comments)
            .context("Failed to write bendis config.toml")?;
        Ok(())
    }
}

/// Get the global config file path
/// Returns: ~/.config/bendis/config.toml on Linux/macOS
///          %APPDATA%\bendis\config.toml on Windows
pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Failed to get config directory")?;
    Ok(config_dir.join("bendis").join(CONFIG_FILE_NAME))
}

/// Get the bendis_workspace directory path in current project
pub fn get_bendis_dir() -> PathBuf {
    PathBuf::from("bendis_workspace")
}

/// Get the project root directory path
pub fn get_root_dir() -> PathBuf {
    PathBuf::from(".")
}

/// Required entries for bendis_workspace/.gitignore
const REQUIRED_GITIGNORE_ENTRIES: &[&str] = &[
    ".bender/",
];

/// Default .gitignore content with header comment
const DEFAULT_GITIGNORE_CONTENT: &str = "# Auto-managed by bendis
# WARNING: Do not remove the entries below, they are required for proper git tracking
# You can add your own entries after this section

.bender/

";

/// Check if bendis_workspace/.gitignore contains all required entries
/// Returns Ok(()) if all required entries exist, or Err with missing entries
pub fn check_bendis_gitignore() -> Result<()> {
    let gitignore_path = get_bendis_dir().join(".gitignore");

    // If .gitignore doesn't exist, create it with default content
    if !gitignore_path.exists() {
        fs::write(&gitignore_path, DEFAULT_GITIGNORE_CONTENT)
            .context("Failed to create bendis_workspace/.gitignore")?;
        return Ok(());
    }

    // Read existing content
    let existing_content = fs::read_to_string(&gitignore_path)
        .context("Failed to read bendis_workspace/.gitignore")?;

    // Check if all required entries are present
    let missing_entries: Vec<&str> = REQUIRED_GITIGNORE_ENTRIES
        .iter()
        .filter(|&&entry| !existing_content.lines().any(|line| line.trim() == entry))
        .copied()
        .collect();

    if !missing_entries.is_empty() {
        anyhow::bail!(
            "Missing required entries in bendis_workspace/.gitignore: {}",
            missing_entries.join(", ")
        );
    }

    Ok(())
}

/// Create bendis_workspace/.gitignore with default content (used by init command)
pub fn create_bendis_gitignore() -> Result<()> {
    let gitignore_path = get_bendis_dir().join(".gitignore");
    fs::write(&gitignore_path, DEFAULT_GITIGNORE_CONTENT)
        .context("Failed to create bendis_workspace/.gitignore")?;
    Ok(())
}

/// Header comment for bendis_workspace .gitignore managed section
const BENDIS_WORKSPACE_GITIGNORE_HEADER: &str = "\n# Auto-managed by bendis
# WARNING: Do not remove the entries below, they are required for proper git tracking
# You can add your own entries after this section\n";

/// Check and update bendis_workspace .gitignore to ensure it contains detected directories
/// Automatically detects directories from Bender YAML files
/// Returns Ok(()) if entries are present or successfully added
pub fn ensure_bendis_workspace_gitignore_entries() -> Result<()> {
    let gitignore_path = get_bendis_dir().join(".gitignore");

    // Get directories from YAML files
    let dirs_to_ignore = extract_path_dependencies()?;
    let entries_to_add: Vec<String> = dirs_to_ignore.iter().map(|d| format!("{}/", d)).collect();

    // Read existing content or create empty if doesn't exist
    let existing_content = if gitignore_path.exists() {
        fs::read_to_string(&gitignore_path)
            .context("Failed to read bendis_workspace/.gitignore")?
    } else {
        String::new()
    };

    // Check which entries are missing
    let missing_entries: Vec<&str> = entries_to_add
        .iter()
        .filter(|entry| !existing_content.lines().any(|line| line.trim() == entry.as_str()))
        .map(|s| s.as_str())
        .collect();

    // If all entries exist, nothing to do
    if missing_entries.is_empty() {
        return Ok(());
    }

    // Build the new content to append
    let mut new_content = existing_content;

    // Add header comment if we're adding entries
    new_content.push_str(BENDIS_WORKSPACE_GITIGNORE_HEADER);

    // Add missing entries
    for entry in missing_entries {
        new_content.push_str(entry);
        new_content.push('\n');
    }

    // Write back to file
    fs::write(&gitignore_path, new_content)
        .context("Failed to update bendis_workspace/.gitignore")?;

    Ok(())
}

/// Remove detected directory entries from root .gitignore if they exist
/// This ensures the root directories are tracked by git
/// Automatically detects directories from Bender YAML files
pub fn ensure_root_gitignore_excludes_hw_target() -> Result<()> {
    let gitignore_path = get_root_dir().join(".gitignore");

    // If .gitignore doesn't exist, nothing to do
    if !gitignore_path.exists() {
        return Ok(());
    }

    // Get directories from YAML files
    let dirs_to_exclude = extract_path_dependencies()?;
    let entries_to_remove: Vec<String> = dirs_to_exclude.iter().map(|d| format!("{}/", d)).collect();

    let existing_content = fs::read_to_string(&gitignore_path)
        .context("Failed to read root .gitignore")?;

    // Filter out detected directory entries
    let new_content: String = existing_content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !entries_to_remove.iter().any(|entry| trimmed == entry)
        })
        .collect::<Vec<&str>>()
        .join("\n");

    // Only write if content changed
    if new_content != existing_content.trim_end() {
        fs::write(&gitignore_path, new_content + "\n")
            .context("Failed to update root .gitignore")?;
    }

    Ok(())
}

/// Copy directories from root directory to bendis_workspace/
/// Automatically detects directories from Bender YAML files
/// Uses hash comparison to skip copying if content hasn't changed
pub fn copy_root_dirs_to_bendis_workspace() -> Result<()> {
    let root_dir = get_root_dir();
    let bendis_dir = get_bendis_dir();

    // Get directories to copy from YAML files (or default to hw, target)
    let dirs_to_copy = extract_path_dependencies()?;

    for dir_name in &dirs_to_copy {
        let src_dir = root_dir.join(dir_name);
        let dest_dir = bendis_dir.join(dir_name);

        // Only copy if source exists
        if src_dir.exists() {
            // Check if destination exists and compare hashes
            let needs_copy = if dest_dir.exists() {
                // Calculate hash for both directories
                let src_hash = calculate_dir_hash(&src_dir)?;
                let dest_hash = calculate_dir_hash(&dest_dir)?;

                // Only copy if hashes differ
                src_hash != dest_hash
            } else {
                // Destination doesn't exist, need to copy
                true
            };

            if needs_copy {
                // Remove destination if it exists
                if dest_dir.exists() {
                    fs::remove_dir_all(&dest_dir)
                        .with_context(|| format!("Failed to remove existing bendis_workspace/{}/", dir_name))?;
                }

                // Copy directory recursively
                copy_dir_recursive(&src_dir, &dest_dir)
                    .with_context(|| format!("Failed to copy root {}/ to bendis_workspace/", dir_name))?;
            }
        }
    }

    Ok(())
}

/// Calculate a hash for a directory based on file paths, sizes, and modification times
/// This is faster than hashing file contents and sufficient for detecting changes
fn calculate_dir_hash(dir: &Path) -> Result<String> {
    use sha2::{Digest, Sha256};
    use std::collections::BTreeMap;

    let mut hasher = Sha256::new();
    let mut entries = BTreeMap::new();

    // Collect all files with their metadata (sorted by path for consistency)
    collect_files_metadata(dir, dir, &mut entries)?;

    // Hash each entry in sorted order
    for (rel_path, (size, mtime)) in entries {
        hasher.update(rel_path.as_bytes());
        hasher.update(&size.to_le_bytes());
        hasher.update(&mtime.to_le_bytes());
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// Recursively collect file metadata for hash calculation
fn collect_files_metadata(
    base: &Path,
    current: &Path,
    entries: &mut std::collections::BTreeMap<String, (u64, i64)>,
) -> Result<()> {
    for entry in fs::read_dir(current)
        .with_context(|| format!("Failed to read directory {}", current.display()))?
    {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_files_metadata(base, &path, entries)?;
        } else {
            let metadata = entry.metadata()?;
            let size = metadata.len();
            let mtime = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);

            // Store relative path from base directory
            let rel_path = path
                .strip_prefix(base)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();

            entries.insert(rel_path, (size, mtime));
        }
    }

    Ok(())
}

/// Helper function to recursively copy a directory
fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<()> {
    // Create destination directory
    fs::create_dir_all(dest)
        .with_context(|| format!("Failed to create directory {}", dest.display()))?;

    // Iterate over entries in source directory
    for entry in fs::read_dir(src)
        .with_context(|| format!("Failed to read directory {}", src.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let dest_path = dest.join(&file_name);

        if path.is_dir() {
            // Recursively copy subdirectory
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            // Copy file
            fs::copy(&path, &dest_path)
                .with_context(|| format!("Failed to copy {} to {}",
                    path.display(), dest_path.display()))?;
        }
    }

    Ok(())
}

/// Check if legacy .bendis directory exists with complete structure
pub fn detect_legacy_structure() -> bool {
    let legacy_dir = Path::new(".bendis");

    if !legacy_dir.exists() || !legacy_dir.is_dir() {
        return false;
    }

    // Check if all required files exist
    let bender_yml = legacy_dir.join("Bender.yml");
    let dot_bender_yml = legacy_dir.join(".bender.yml");
    let bender_lock = legacy_dir.join("Bender.lock");

    bender_yml.exists() && dot_bender_yml.exists() && bender_lock.exists()
}

/// Prompt user for migration confirmation
pub fn prompt_migration() -> Result<bool> {
    use colored::Colorize;

    println!("\n{}", "━".repeat(60).yellow());
    println!("{}", "  Detected legacy Bendis structure".bold().yellow());
    println!("{}", "━".repeat(60).yellow());
    println!();
    println!("Bendis {} uses a new workspace directory name:", "≥v0.3.0".cyan());
    println!("  {} → {}", ".bendis/".red().strikethrough(), "bendis_workspace/".green().bold());
    println!();
    println!("Would you like to upgrade your project structure?");
    println!();
    println!("{}", "Migration details:".bold());
    println!("  {} Your existing .bendis/ will be renamed to bendis_workspace/", "•".cyan());
    println!("  {} All files and settings will be preserved", "•".cyan());
    println!("  {} This is a one-time migration", "•".cyan());
    println!();
    print!("Migrate to new structure? [{}]: ", "Y/n".bold());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    Ok(input.is_empty() || input == "y" || input == "yes")
}

/// Migrate legacy .bendis directory to bendis_workspace
pub fn migrate_legacy_to_workspace() -> Result<()> {
    use colored::Colorize;

    let legacy_dir = Path::new(".bendis");
    let new_dir = get_bendis_dir();

    println!();
    println!("{}", "Starting migration...".bold());

    // Safety check: ensure new directory doesn't exist
    if new_dir.exists() {
        anyhow::bail!(
            "Migration failed: {} already exists. Please remove it first or backup your data.",
            new_dir.display()
        );
    }

    // Step 1: Copy entire directory
    println!("  {} Copying .bendis/ to bendis_workspace/...", "→".blue());
    copy_dir_recursive(&legacy_dir, &new_dir)
        .context("Failed to copy legacy directory")?;

    // Step 2: Verify the copy
    println!("  {} Verifying migration...", "→".blue());
    let verify_files = ["Bender.yml", ".bender.yml", "Bender.lock"];
    for file in &verify_files {
        let new_file = new_dir.join(file);
        if !new_file.exists() {
            anyhow::bail!("Migration verification failed: {} not found", file);
        }
    }

    // Step 3: Remove old directory
    println!("  {} Removing old .bendis/...", "→".blue());
    fs::remove_dir_all(&legacy_dir)
        .context("Failed to remove legacy .bendis directory")?;

    println!("  {} Migration completed successfully!", "✓".green());
    println!();
    println!("{}", "Your project has been upgraded to the new structure.".green());
    println!("You can now use {} as usual.", "bendis update".cyan().bold());
    println!();

    Ok(())
}

/// Check and handle legacy migration at startup
/// Returns true if migration was performed or declined
pub fn check_and_migrate_if_needed() -> Result<bool> {
    if !detect_legacy_structure() {
        return Ok(false);
    }

    // Legacy structure detected, prompt user
    if prompt_migration()? {
        migrate_legacy_to_workspace()?;
        Ok(true)
    } else {
        use colored::Colorize;
        println!();
        println!("{}", "Migration skipped.".yellow());
        println!("Note: You can migrate later by running {} again.", "bendis init".cyan());
        println!();
        Ok(true)
    }
}

/// Extract top-level directories from path dependencies in Bender YAML files
/// Returns a Vec of unique top-level directory names (e.g., ["hw", "target", "hardware"])
pub fn extract_path_dependencies() -> Result<Vec<String>> {
    use serde_yaml::Value;
    use std::collections::HashSet;

    let bendis_dir = get_bendis_dir();
    let bender_yml = bendis_dir.join("Bender.yml");
    let dot_bender_yml = bendis_dir.join(".bender.yml");

    let mut top_level_dirs = HashSet::new();

    // Parse Bender.yml if it exists
    if bender_yml.exists() {
        let content = fs::read_to_string(&bender_yml)
            .context("Failed to read Bender.yml")?;

        if let Ok(yaml) = serde_yaml::from_str::<Value>(&content) {
            // Extract paths from dependencies section
            if let Some(deps) = yaml.get("dependencies").and_then(|v| v.as_mapping()) {
                for (_, dep_value) in deps {
                    if let Some(path) = dep_value.get("path").and_then(|p| p.as_str()) {
                        if let Some(top_dir) = extract_top_level_dir(path) {
                            top_level_dirs.insert(top_dir);
                        }
                    }
                }
            }
        }
    }

    // Parse .bender.yml if it exists
    if dot_bender_yml.exists() {
        let content = fs::read_to_string(&dot_bender_yml)
            .context("Failed to read .bender.yml")?;

        if let Ok(yaml) = serde_yaml::from_str::<Value>(&content) {
            // Extract paths from overrides section
            if let Some(overrides) = yaml.get("overrides").and_then(|v| v.as_mapping()) {
                for (_, override_value) in overrides {
                    if let Some(path) = override_value.get("path").and_then(|p| p.as_str()) {
                        if let Some(top_dir) = extract_top_level_dir(path) {
                            top_level_dirs.insert(top_dir);
                        }
                    }
                }
            }
        }
    }

    // If no directories found, fallback to default
    if top_level_dirs.is_empty() {
        Ok(vec!["hw".to_string(), "target".to_string()])
    } else {
        let mut dirs: Vec<String> = top_level_dirs.into_iter().collect();
        dirs.sort();
        Ok(dirs)
    }
}

/// Extract the top-level directory from a path string
/// E.g., "hw/padframe/pulpissimo" -> Some("hw")
///       "target/sim/vip" -> Some("target")
fn extract_top_level_dir(path: &str) -> Option<String> {
    path.split('/')
        .next()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}
