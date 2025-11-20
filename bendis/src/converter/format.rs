use anyhow::{Context, Result};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Clone)]
enum Source {
    Git(String),
    Path(String),
}

impl<'de> Deserialize<'de> for Source {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map = HashMap::<String, String>::deserialize(deserializer)?;

        if let Some(git_url) = map.get("Git") {
            Ok(Source::Git(git_url.clone()))
        } else if let Some(path) = map.get("Path") {
            Ok(Source::Path(path.clone()))
        } else {
            Err(serde::de::Error::custom("Expected 'Git' or 'Path' key in source"))
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Package {
    revision: Option<String>,
    version: Option<String>,
    source: Source,
    dependencies: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LockFile {
    packages: HashMap<String, Package>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
enum DependencySpec {
    Simple(String),
    Detailed(HashMap<String, serde_yaml::Value>),
}

#[derive(Debug, Deserialize, Serialize)]
struct BenderYml {
    dependencies: Option<HashMap<String, DependencySpec>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DotBenderYml {
    overrides: Option<HashMap<String, HashMap<String, serde_yaml::Value>>>,
}

/// Convert GitHub URL to IHEP internal Git URL
fn convert_url(git_url: &str) -> String {
    if git_url.contains("github.com/pulp-platform") {
        // Extract repository name
        if let Some(repo_name) = git_url.split('/').last() {
            return format!("git@code.ihep.ac.cn:heris/heris-platform/{}", repo_name);
        }
    }
    git_url.to_string()
}

/// Extract dependencies from lock file
fn extract_dependencies_from_lock(lock_data: &LockFile) -> HashMap<String, HashMap<String, serde_yaml::Value>> {
    let mut dependencies = HashMap::new();

    for (pkg_name, pkg_info) in &lock_data.packages {
        match &pkg_info.source {
            Source::Git(git_url) => {
                let converted_url = convert_url(git_url);
                let mut dep_info = HashMap::new();
                dep_info.insert("git".to_string(), serde_yaml::Value::String(converted_url));

                // Add version or revision
                if let Some(version) = &pkg_info.version {
                    dep_info.insert("version".to_string(), serde_yaml::to_value(version).unwrap());
                } else if let Some(revision) = &pkg_info.revision {
                    dep_info.insert("rev".to_string(), serde_yaml::Value::String(revision.clone()));
                }

                dependencies.insert(pkg_name.clone(), dep_info);
            }
            Source::Path(path) => {
                let mut dep_info = HashMap::new();
                dep_info.insert("path".to_string(), serde_yaml::Value::String(path.clone()));
                dependencies.insert(pkg_name.clone(), dep_info);
            }
        }
    }

    dependencies
}

/// Extract dependency names from Bender.yml
fn extract_dependencies_from_yml(yml_data: &BenderYml) -> HashSet<String> {
    if let Some(deps) = &yml_data.dependencies {
        deps.keys().cloned().collect()
    } else {
        HashSet::new()
    }
}

/// Extract existing overrides from .bender.yml
fn extract_overrides_from_bender_yml(bender_yml_data: &DotBenderYml) -> HashMap<String, HashMap<String, serde_yaml::Value>> {
    if let Some(overrides) = &bender_yml_data.overrides {
        overrides.clone()
    } else {
        HashMap::new()
    }
}

/// Compare two version strings
fn compare_versions(v1: &Option<String>, v2: &Option<String>) -> i8 {
    match (v1, v2) {
        (None, None) => 0,
        (None, Some(_)) => -1,
        (Some(_), None) => 1,
        (Some(v1_str), Some(v2_str)) => {
            let v1_parts: Vec<&str> = v1_str.split('.').collect();
            let v2_parts: Vec<&str> = v2_str.split('.').collect();

            let max_len = v1_parts.len().max(v2_parts.len());

            for i in 0..max_len {
                let p1 = v1_parts.get(i).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
                let p2 = v2_parts.get(i).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);

                if p1 > p2 {
                    return 1;
                } else if p1 < p2 {
                    return -1;
                }
            }
            0
        }
    }
}

/// Find missing dependencies that need to be added to overrides
fn find_missing_dependencies(
    lock_deps: &HashMap<String, HashMap<String, serde_yaml::Value>>,
    yml_deps: &HashSet<String>,
    existing_overrides: &HashMap<String, HashMap<String, serde_yaml::Value>>,
) -> HashMap<String, HashMap<String, serde_yaml::Value>> {
    let mut missing_deps = HashMap::new();

    for (dep_name, dep_info) in lock_deps {
        // Skip if already in Bender.yml
        if yml_deps.contains(dep_name) {
            continue;
        }

        // Check if already in overrides
        if let Some(existing_info) = existing_overrides.get(dep_name) {
            // Both have git URLs, check versions
            if dep_info.contains_key("git") && existing_info.contains_key("git") {
                let new_version = dep_info.get("version").and_then(|v| {
                    if let serde_yaml::Value::String(s) = v {
                        Some(s.clone())
                    } else {
                        v.as_f64().map(|f| f.to_string())
                    }
                });

                let existing_version = existing_info.get("version").and_then(|v| {
                    if let serde_yaml::Value::String(s) = v {
                        Some(s.clone())
                    } else {
                        v.as_f64().map(|f| f.to_string())
                    }
                });

                // If new version is newer, update it
                if compare_versions(&new_version, &existing_version) > 0 {
                    missing_deps.insert(dep_name.clone(), dep_info.clone());
                }
            } else if dep_info.contains_key("git") && existing_info.contains_key("path") {
                // Prefer git over path
                missing_deps.insert(dep_name.clone(), dep_info.clone());
            }
        } else {
            // Not in overrides, add it
            missing_deps.insert(dep_name.clone(), dep_info.clone());
        }
    }

    missing_deps
}

/// Format a single dependency line in inline YAML style
fn format_dependency_line(
    name: &str,
    dep_info: &HashMap<String, serde_yaml::Value>,
    max_name_len: usize,
    max_url_len: usize,
) -> String {
    let name_padding = " ".repeat(max_name_len.saturating_sub(name.len()));

    if let Some(serde_yaml::Value::String(path)) = dep_info.get("path") {
        let url_part = format!(r#"{{ path: "{}" "#, path);
        let url_padding = " ".repeat(max_url_len.saturating_sub(url_part.len()));
        format!("  {}:{} {}{}}}", name, name_padding, url_part, url_padding)
    } else if let Some(serde_yaml::Value::String(git_url)) = dep_info.get("git") {
        if let Some(version) = dep_info.get("version") {
            let version_str = match version {
                serde_yaml::Value::String(s) => s.clone(),
                serde_yaml::Value::Number(n) => n.to_string(),
                _ => format!("{:?}", version),
            };
            let git_part = format!(r#"{{ git: "{}","#, git_url);
            let url_padding = " ".repeat(max_url_len.saturating_sub(git_part.len()));
            format!("  {}:{} {}{} version: {} }}", name, name_padding, git_part, url_padding, version_str)
        } else if let Some(serde_yaml::Value::String(rev)) = dep_info.get("rev") {
            let git_part = format!(r#"{{ git: "{}","#, git_url);
            let url_padding = " ".repeat(max_url_len.saturating_sub(git_part.len()));
            format!(r#"  {}:{} {}{} rev: "{}" }}"#, name, name_padding, git_part, url_padding, rev)
        } else {
            let git_part = format!(r#"{{ git: "{}"#, git_url);
            let url_padding = " ".repeat(max_url_len.saturating_sub(git_part.len()));
            format!("  {}:{} {}{} }}", name, name_padding, git_part, url_padding)
        }
    } else {
        format!("  {}:{} {{}}", name, name_padding)
    }
}

/// Generate formatted override lines
fn generate_overrides_lines(all_overrides: &HashMap<String, HashMap<String, serde_yaml::Value>>) -> Vec<String> {
    if all_overrides.is_empty() {
        return Vec::new();
    }

    // Calculate maximum name length
    let max_name_len = all_overrides.keys().map(|k| k.len()).max().unwrap_or(0);

    // Calculate maximum URL part length
    let mut max_url_len = 0;
    for dep_info in all_overrides.values() {
        let url_part_len = if let Some(serde_yaml::Value::String(path)) = dep_info.get("path") {
            format!(r#"{{ path: "{}" "#, path).len()
        } else if let Some(serde_yaml::Value::String(git_url)) = dep_info.get("git") {
            if dep_info.contains_key("version") || dep_info.contains_key("rev") {
                format!(r#"{{ git: "{}","#, git_url).len()
            } else {
                format!(r#"{{ git: "{}"#, git_url).len()
            }
        } else {
            0
        };
        max_url_len = max_url_len.max(url_part_len);
    }

    let mut lines = Vec::new();
    for (name, dep_info) in all_overrides {
        lines.push(format_dependency_line(name, dep_info, max_name_len, max_url_len));
    }

    lines
}

/// Update .bender.yml with new overrides
fn update_bender_yml_overrides(
    bender_yml_text: &str,
    existing_overrides: &HashMap<String, HashMap<String, serde_yaml::Value>>,
    new_overrides: &HashMap<String, HashMap<String, serde_yaml::Value>>,
) -> String {
    // Merge overrides
    let mut all_overrides = existing_overrides.clone();
    all_overrides.extend(new_overrides.clone());

    // Generate formatted override lines
    let override_lines = generate_overrides_lines(&all_overrides);

    if override_lines.is_empty() {
        return bender_yml_text.to_string();
    }

    // Build new overrides section
    let new_overrides_section = format!("overrides:\n{}", override_lines.join("\n"));

    // Replace overrides section - find "overrides:" and everything until the next top-level key or end
    // Split into lines and rebuild, replacing the overrides section
    let lines: Vec<&str> = bender_yml_text.lines().collect();
    let mut result = Vec::new();
    let mut in_overrides = false;
    let mut overrides_found = false;

    for line in lines {
        if line.starts_with("overrides:") {
            // Found the overrides section, replace it
            result.push(new_overrides_section.as_str());
            in_overrides = true;
            overrides_found = true;
        } else if in_overrides {
            // Check if this is a new top-level section (starts with a letter and ends with :)
            if !line.is_empty() && !line.starts_with(' ') && !line.starts_with('\t') {
                // New section, stop skipping
                in_overrides = false;
                result.push(line);
            }
            // Otherwise skip this line (it's part of the old overrides)
        } else {
            result.push(line);
        }
    }

    // If we didn't find an overrides section, append it at the end
    if !overrides_found {
        result.push("\n");
        result.push(new_overrides_section.as_str());
    }

    result.join("\n")
}

/// Main conversion function
pub fn convert(
    bendis_dir: &Path,
    root_dir: &Path,
) -> Result<()> {
    // Read input files
    let lock_path = bendis_dir.join("Bender.lock");
    let yml_path = bendis_dir.join("Bender.yml");
    let bender_yml_path = bendis_dir.join(".bender.yml");

    let lock_content = fs::read_to_string(&lock_path)
        .context("Failed to read bendis_workspace/Bender.lock")?;
    let lock_data: LockFile = serde_yaml::from_str(&lock_content)
        .context("Failed to parse Bender.lock")?;

    let yml_content = fs::read_to_string(&yml_path)
        .context("Failed to read bendis_workspace/Bender.yml")?;
    let yml_data: BenderYml = serde_yaml::from_str(&yml_content)
        .unwrap_or(BenderYml { dependencies: None });

    let bender_yml_text = fs::read_to_string(&bender_yml_path)
        .context("Failed to read bendis_workspace/.bender.yml")?;
    let bender_yml_data: DotBenderYml = serde_yaml::from_str(&bender_yml_text)
        .unwrap_or(DotBenderYml { overrides: None });

    // Extract dependencies
    let lock_deps = extract_dependencies_from_lock(&lock_data);
    let yml_deps = extract_dependencies_from_yml(&yml_data);
    let existing_overrides = extract_overrides_from_bender_yml(&bender_yml_data);

    // Find missing dependencies
    let missing_deps = find_missing_dependencies(&lock_deps, &yml_deps, &existing_overrides);

    // Copy Bender.yml to root
    fs::copy(&yml_path, root_dir.join("Bender.yml"))
        .context("Failed to copy Bender.yml to root")?;

    // Update .bender.yml
    let updated_bender_yml = update_bender_yml_overrides(&bender_yml_text, &existing_overrides, &missing_deps);

    // Write to root
    let output_path = root_dir.join(".bender.yml");
    fs::write(&output_path, updated_bender_yml)
        .context("Failed to write .bender.yml to root")?;

    Ok(())
}
