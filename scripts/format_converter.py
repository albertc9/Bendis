#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Bender format converter tool
Functions:
1. Read Bender.lock and Bender.yml files
2. Compare dependencies and find missing ones
3. Generate .bender.yml with missing dependencies added to overrides section
4. Replace GitHub URLs with IHEP internal Git URLs
5. Handle version conflicts by using newer versions
"""

import yaml
import os
import sys
import re
from typing import Dict, Any, Set, Tuple


def load_lock_file(lock_path: str) -> Dict[str, Any]:
    """Load Bender.lock file"""
    with open(lock_path, 'r', encoding='utf-8') as f:
        return yaml.safe_load(f)


def load_yml_file(yml_path: str) -> Dict[str, Any]:
    """Load YAML file"""
    with open(yml_path, 'r', encoding='utf-8') as f:
        return yaml.safe_load(f)


def load_yml_file_as_text(yml_path: str) -> str:
    """Load YAML file as raw text"""
    with open(yml_path, 'r', encoding='utf-8') as f:
        return f.read()


def convert_url(git_url: str) -> str:
    """Convert GitHub URL to IHEP internal Git URL"""
    if 'github.com/pulp-platform' in git_url:
        # Extract repository name
        repo_name = git_url.split('/')[-1]
        # Convert to IHEP internal URL
        return f"git@code.ihep.ac.cn:heris/heris-platform/{repo_name}"
    return git_url


def extract_dependencies_from_lock(lock_data: Dict[str, Any]) -> Dict[str, Dict[str, Any]]:
    """
    Extract dependency information from lock file

    Args:
        lock_data: Lock file data

    Returns:
        Dependencies dictionary with version or revision info
    """
    dependencies = {}
    packages = lock_data.get('packages', {})

    for pkg_name, pkg_info in packages.items():
        source = pkg_info.get('source', {})
        version = pkg_info.get('version')
        revision = pkg_info.get('revision')

        # Process Git-type dependencies
        if 'Git' in source:
            git_url = source['Git']
            # Convert URL
            converted_url = convert_url(git_url)

            dep_info = {'git': converted_url}

            # Add version info if it exists and is not null
            # Otherwise, use revision with 'rev' label
            if version is not None:
                dep_info['version'] = version
            elif revision is not None:
                dep_info['rev'] = revision

            dependencies[pkg_name] = dep_info
        elif 'Path' in source:
            # Process Path-type dependencies
            dependencies[pkg_name] = {'path': source['Path']}

    return dependencies


def extract_dependencies_from_yml(yml_data: Dict[str, Any]) -> Set[str]:
    """
    Extract dependency names from Bender.yml

    Args:
        yml_data: Bender.yml data

    Returns:
        Set of dependency names
    """
    dependencies = yml_data.get('dependencies', {})
    if dependencies:
        return set(dependencies.keys())
    return set()


def extract_overrides_from_bender_yml(bender_yml_data: Dict[str, Any]) -> Dict[str, Dict[str, Any]]:
    """
    Extract existing overrides from .bender.yml

    Args:
        bender_yml_data: .bender.yml data

    Returns:
        Dictionary of existing overrides
    """
    overrides = bender_yml_data.get('overrides', {})
    if overrides:
        return dict(overrides)
    return {}


def compare_versions(version1: Any, version2: Any) -> int:
    """
    Compare two version numbers

    Args:
        version1: First version (can be string, float, or None)
        version2: Second version (can be string, float, or None)

    Returns:
        1 if version1 > version2, -1 if version1 < version2, 0 if equal
    """
    # Handle None cases
    if version1 is None and version2 is None:
        return 0
    if version1 is None:
        return -1
    if version2 is None:
        return 1

    # Convert to string for comparison
    v1_str = str(version1)
    v2_str = str(version2)

    # Split by dots and compare
    v1_parts = v1_str.split('.')
    v2_parts = v2_str.split('.')

    # Pad to same length
    max_len = max(len(v1_parts), len(v2_parts))
    v1_parts += ['0'] * (max_len - len(v1_parts))
    v2_parts += ['0'] * (max_len - len(v2_parts))

    for p1, p2 in zip(v1_parts, v2_parts):
        try:
            n1 = int(p1)
            n2 = int(p2)
            if n1 > n2:
                return 1
            elif n1 < n2:
                return -1
        except ValueError:
            # If not numbers, compare as strings
            if p1 > p2:
                return 1
            elif p1 < p2:
                return -1

    return 0


def find_missing_dependencies(lock_deps: Dict[str, Dict[str, Any]],
                              yml_deps: Set[str],
                              existing_overrides: Dict[str, Dict[str, Any]]) -> Dict[str, Dict[str, Any]]:
    """
    Find dependencies that are in lock but not in yml, handling version conflicts

    Args:
        lock_deps: Dependencies from Bender.lock
        yml_deps: Dependency names from Bender.yml
        existing_overrides: Existing overrides from .bender.yml

    Returns:
        Dictionary of missing/updated dependencies to add to overrides
    """
    missing_deps = {}

    for dep_name, dep_info in lock_deps.items():
        # Skip if already in Bender.yml
        if dep_name in yml_deps:
            continue

        # Check if already in overrides
        if dep_name in existing_overrides:
            existing_info = existing_overrides[dep_name]

            # If both have git URLs, check versions/revisions
            if 'git' in dep_info and 'git' in existing_info:
                new_version = dep_info.get('version')
                existing_version = existing_info.get('version')
                new_rev = dep_info.get('rev')
                existing_rev = existing_info.get('rev')

                # If both have versions, compare them
                if new_version is not None and existing_version is not None:
                    if compare_versions(new_version, existing_version) > 0:
                        missing_deps[dep_name] = dep_info
                # If new has version but existing has rev, prefer version
                elif new_version is not None and existing_version is None:
                    missing_deps[dep_name] = dep_info
                # If both have rev, compare rev strings (use newer if different)
                elif new_rev is not None and existing_rev is not None:
                    if new_rev != existing_rev:
                        missing_deps[dep_name] = dep_info
                # If new has rev but existing has version, keep existing (prefer version)
                # Otherwise skip (same version/rev or older)
            # If one is path and one is git, prefer git (from lock)
            elif 'git' in dep_info and 'path' in existing_info:
                missing_deps[dep_name] = dep_info
        else:
            # Not in overrides, add it
            missing_deps[dep_name] = dep_info

    return missing_deps


def format_dependency_line(name: str, dep_info: Dict[str, Any], max_name_len: int, max_url_len: int) -> str:
    """
    Format a single dependency line in inline YAML style

    Args:
        name: Dependency name
        dep_info: Dependency information (git/path and version/rev)
        max_name_len: Maximum name length for alignment
        max_url_len: Maximum URL length for alignment

    Returns:
        Formatted dependency line
    """
    # Calculate padding for name alignment
    name_padding = ' ' * (max_name_len - len(name))

    if 'path' in dep_info:
        # Path-type dependency
        path = dep_info["path"]
        url_padding = ' ' * (max_url_len - len(f'{{ path: "{path}"'))
        return f'  {name}:{name_padding} {{ path: "{path}"{url_padding} }}'
    elif 'git' in dep_info:
        # Git-type dependency
        git_url = dep_info['git']
        git_part = f'{{ git: "{git_url}"'

        if 'version' in dep_info:
            version = dep_info['version']
            git_part_with_comma = f'{git_part},'
            url_padding = ' ' * (max_url_len - len(git_part_with_comma))
            return f'  {name}:{name_padding} {git_part_with_comma}{url_padding} version: {version} }}'
        elif 'rev' in dep_info:
            rev = dep_info['rev']
            git_part_with_comma = f'{git_part},'
            url_padding = ' ' * (max_url_len - len(git_part_with_comma))
            return f'  {name}:{name_padding} {git_part_with_comma}{url_padding} rev: "{rev}" }}'
        else:
            url_padding = ' ' * (max_url_len - len(git_part))
            return f'  {name}:{name_padding} {git_part}{url_padding} }}'

    return f'  {name}:{name_padding} {{}}'


def generate_overrides_lines(all_overrides: Dict[str, Dict[str, Any]]) -> list:
    """
    Generate formatted override lines

    Args:
        all_overrides: All overrides to format

    Returns:
        List of formatted lines
    """
    if not all_overrides:
        return []

    # Calculate maximum name length for alignment
    max_name_len = max(len(name) for name in all_overrides.keys())

    # Calculate maximum URL part length for alignment
    max_url_len = 0
    for dep_info in all_overrides.values():
        if 'path' in dep_info:
            url_part_len = len(f'{{ path: "{dep_info["path"]}"')
        elif 'git' in dep_info:
            git_url = dep_info['git']
            if 'version' in dep_info or 'rev' in dep_info:
                url_part_len = len(f'{{ git: "{git_url}",')
            else:
                url_part_len = len(f'{{ git: "{git_url}"')
        else:
            url_part_len = 0
        max_url_len = max(max_url_len, url_part_len)

    lines = []
    for name, dep_info in all_overrides.items():
        lines.append(format_dependency_line(name, dep_info, max_name_len, max_url_len))

    return lines


def update_bender_yml_overrides(bender_yml_text: str,
                                existing_overrides: Dict[str, Dict[str, Any]],
                                new_overrides: Dict[str, Dict[str, Any]]) -> str:
    """
    Update .bender.yml with new overrides

    Args:
        bender_yml_text: Original .bender.yml content
        existing_overrides: Existing overrides
        new_overrides: New overrides to add/update

    Returns:
        Updated .bender.yml content
    """
    # Merge overrides (new ones override existing ones with same name)
    all_overrides = existing_overrides.copy()
    all_overrides.update(new_overrides)

    # Generate formatted override lines
    override_lines = generate_overrides_lines(all_overrides)

    if not override_lines:
        return bender_yml_text

    # Build new overrides section
    new_overrides_section = "overrides:\n" + '\n'.join(override_lines)

    # Replace overrides section in the original text
    # Pattern matches from "overrides:" to the next non-indented section or EOF
    pattern = r'overrides:.*?(?=\n[a-z_]+:|$)'

    updated_text = re.sub(pattern, new_overrides_section, bender_yml_text, flags=re.DOTALL)

    return updated_text


def save_file(content: str, output_path: str):
    """Save file"""
    # Ensure output directory exists
    output_dir = os.path.dirname(output_path)
    os.makedirs(output_dir, exist_ok=True)

    with open(output_path, 'w', encoding='utf-8') as f:
        f.write(content)


def process_bender_files(timestamp: str, input_base: str = "input/filepack", output_base: str = "output/filepackout"):
    """
    Process Bender files for the specified timestamp

    Args:
        timestamp: Timestamp, e.g. "20251115"
        input_base: Input files base path
        output_base: Output files base path
    """
    # Build input file paths
    lock_path = os.path.join(input_base, timestamp, "Bender.lock")
    yml_path = os.path.join(input_base, timestamp, "Bender.yml")
    bender_yml_path = os.path.join(input_base, timestamp, ".bender.yml")

    # Check if files exist
    if not os.path.exists(lock_path):
        raise FileNotFoundError(f"Lock file does not exist: {lock_path}")
    if not os.path.exists(yml_path):
        raise FileNotFoundError(f"YML file does not exist: {yml_path}")
    if not os.path.exists(bender_yml_path):
        raise FileNotFoundError(f".bender.yml file does not exist: {bender_yml_path}")

    print(f"Processing timestamp: {timestamp}")
    print(f"Reading file: {lock_path}")
    print(f"Reading file: {yml_path}")
    print(f"Reading file: {bender_yml_path}")

    # Load files
    lock_data = load_lock_file(lock_path)
    yml_data = load_yml_file(yml_path)
    bender_yml_data = load_yml_file(bender_yml_path)
    bender_yml_text = load_yml_file_as_text(bender_yml_path)

    # Extract dependencies
    print("Extracting dependencies from Bender.lock...")
    lock_deps = extract_dependencies_from_lock(lock_data)

    print("Extracting dependencies from Bender.yml...")
    yml_deps = extract_dependencies_from_yml(yml_data)

    print("Extracting existing overrides from .bender.yml...")
    existing_overrides = extract_overrides_from_bender_yml(bender_yml_data)

    # Find missing dependencies
    print("Comparing and finding missing/updated dependencies...")
    missing_deps = find_missing_dependencies(lock_deps, yml_deps, existing_overrides)

    if missing_deps:
        print(f"Found {len(missing_deps)} dependencies to add/update in overrides:")
        for dep_name in missing_deps:
            print(f"  - {dep_name}")
    else:
        print("No new dependencies to add.")

    # Update .bender.yml
    print("Updating .bender.yml with new overrides...")
    updated_bender_yml = update_bender_yml_overrides(bender_yml_text, existing_overrides, missing_deps)

    # Save output file
    output_path = os.path.join(output_base, timestamp, ".bender.yml")
    print(f"Saving output file: {output_path}")
    save_file(updated_bender_yml, output_path)

    print(f"Processing complete! Output file: {output_path}")
    print(f"Total dependencies in lock: {len(lock_deps)}")
    print(f"Dependencies in Bender.yml: {len(yml_deps)}")
    print(f"Added/updated in overrides: {len(missing_deps)}")


def main():
    """Main function"""
    if len(sys.argv) < 2:
        print("Usage: python format_converter.py <timestamp>")
        print("Example: python format_converter.py 20251115")
        sys.exit(1)

    timestamp = sys.argv[1]

    try:
        process_bender_files(timestamp)
    except Exception as e:
        print(f"Error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
