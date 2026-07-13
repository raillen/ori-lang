use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageManifest {
    pub root: PathBuf,
    pub manifest_path: PathBuf,
    pub name: String,
    pub version: String,
    pub entry: PathBuf,
    pub ori_version: String,
    pub description: Option<String>,
    pub dependencies: Vec<PackageDependency>,
    pub native_libs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageDependency {
    pub name: String,
    pub requirement: DependencyRequirement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyRequirement {
    Version(String),
    Path {
        path: PathBuf,
        version: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub struct InstallPackageOptions {
    pub name: String,
    pub source: Option<PathBuf>,
    pub cache_root: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
    pub source_root: PathBuf,
    pub installed_root: PathBuf,
    pub already_installed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallPackageOutput {
    pub cache_root: PathBuf,
    pub packages: Vec<InstalledPackage>,
}

pub fn run_install_package(options: InstallPackageOptions) -> Result<InstallPackageOutput, String> {
    let source = match options.source {
        Some(path) => path,
        None => {
            if options.name.starts_with("github.com/")
                || options.name.starts_with("https://")
                || options.name.starts_with("http://")
            {
                let url = if options.name.starts_with("github.com/") {
                    format!("https://{}", options.name)
                } else {
                    options.name.clone()
                };
                let temp_dir = std::env::temp_dir().join(format!(
                    "ori_git_clone_{}",
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis()
                ));

                eprintln!("ori: cloning {}...", url);
                let status = Command::new("git")
                    .arg("clone")
                    .arg("--depth")
                    .arg("1")
                    .arg(&url)
                    .arg(&temp_dir)
                    .status()
                    .map_err(|err| {
                        format!("package.git_clone_failed: failed to invoke git: {err}")
                    })?;

                if !status.success() {
                    return Err(format!(
                        "package.git_clone_failed: git clone failed with status: {}",
                        status
                    ));
                }

                temp_dir
            } else {
                return Err(format!(
                    "package.registry_unavailable: cannot fetch `{}` yet; use `ori install {} --path <local-package>` or provide a GitHub URL",
                    options.name, options.name
                ));
            }
        }
    };

    let cache_root = match options.cache_root {
        Some(path) => path,
        None => default_package_cache_root()?,
    };

    let mut seen = HashSet::new();
    let mut packages = Vec::new();
    let expected_name =
        if options.name.starts_with("github.com/") || options.name.starts_with("http") {
            None
        } else {
            Some(options.name.as_str())
        };

    install_local_package(
        &package_root_from_path(&source)?,
        expected_name,
        &cache_root,
        &mut seen,
        &mut packages,
    )?;

    Ok(InstallPackageOutput {
        cache_root,
        packages,
    })
}

pub fn load_package_manifest(path: impl AsRef<Path>) -> Result<PackageManifest, String> {
    let input = path.as_ref();
    let manifest_path = if input.is_file() {
        input.to_path_buf()
    } else {
        input.join("ori.pkg.toml")
    };

    let source = fs::read_to_string(&manifest_path).map_err(|err| {
        format!(
            "package.manifest_missing: cannot read `{}`: {err}",
            manifest_path.display()
        )
    })?;
    let root = manifest_path
        .parent()
        .ok_or_else(|| {
            format!(
                "package.manifest_invalid: manifest `{}` has no parent directory",
                manifest_path.display()
            )
        })?
        .to_path_buf();

    parse_package_manifest(&source, root, manifest_path)
}

fn install_local_package(
    root: &Path,
    expected_name: Option<&str>,
    cache_root: &Path,
    seen: &mut HashSet<String>,
    packages: &mut Vec<InstalledPackage>,
) -> Result<(), String> {
    let manifest = load_package_manifest(root)?;
    if let Some(expected) = expected_name {
        if manifest.name != expected {
            return Err(format!(
                "package.name_mismatch: requested `{expected}`, but `{}` declares `{}`",
                manifest.manifest_path.display(),
                manifest.name
            ));
        }
    }

    let key = format!("{}@{}", manifest.name, manifest.version);
    if !seen.insert(key) {
        return Ok(());
    }

    for dependency in &manifest.dependencies {
        match &dependency.requirement {
            DependencyRequirement::Version(version) => {
                let cached = cache_root
                    .join(&dependency.name)
                    .join(version)
                    .join("ori.pkg.toml");
                if !cached.is_file() {
                    return Err(format!(
                        "package.registry_unavailable: dependency `{}` requires version `{version}`, but registry fetch is not implemented and `{}` is not cached",
                        dependency.name,
                        cached.display()
                    ));
                }
            }
            DependencyRequirement::Path { path, version } => {
                let dependency_root = manifest.root.join(path);
                let dependency_manifest = load_package_manifest(&dependency_root)?;
                if dependency_manifest.name != dependency.name {
                    return Err(format!(
                        "package.dependency_name_mismatch: dependency `{}` points to package `{}`",
                        dependency.name, dependency_manifest.name
                    ));
                }
                if let Some(expected_version) = version {
                    if dependency_manifest.version != *expected_version {
                        return Err(format!(
                            "package.dependency_version_mismatch: dependency `{}` expected `{expected_version}`, found `{}`",
                            dependency.name, dependency_manifest.version
                        ));
                    }
                }
                install_local_package(
                    &dependency_root,
                    Some(&dependency.name),
                    cache_root,
                    seen,
                    packages,
                )?;
            }
        }
    }

    let target_root = cache_root.join(&manifest.name).join(&manifest.version);
    let already_installed = target_root.join("ori.pkg.toml").is_file();
    if already_installed {
        let cached = load_package_manifest(&target_root)?;
        if cached.name != manifest.name || cached.version != manifest.version {
            return Err(format!(
                "package.cache_conflict: cached package at `{}` does not match `{}` `{}`",
                target_root.display(),
                manifest.name,
                manifest.version
            ));
        }
    } else {
        copy_package_tree(&manifest.root, &target_root)?;
    }

    packages.push(InstalledPackage {
        name: manifest.name,
        version: manifest.version,
        source_root: manifest.root,
        installed_root: target_root,
        already_installed,
    });

    Ok(())
}

fn package_root_from_path(path: &Path) -> Result<PathBuf, String> {
    if path.is_file() {
        return path.parent().map(Path::to_path_buf).ok_or_else(|| {
            format!(
                "package.manifest_invalid: `{}` has no parent directory",
                path.display()
            )
        });
    }
    Ok(path.to_path_buf())
}

fn parse_package_manifest(
    source: &str,
    root: PathBuf,
    manifest_path: PathBuf,
) -> Result<PackageManifest, String> {
    let mut section = String::new();
    let mut package = HashMap::new();
    let mut dependencies = Vec::new();
    let mut native_libs = Vec::new();

    for (line_index, raw_line) in source.lines().enumerate() {
        let line_no = line_index + 1;
        let line = strip_comment(raw_line).trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') {
            if !line.ends_with(']') {
                return Err(format!(
                    "package.manifest_syntax: `{}` line {line_no}: unterminated section",
                    manifest_path.display()
                ));
            }
            section = line[1..line.len() - 1].trim().to_string();
            continue;
        }

        let (key, value) = split_key_value(line).ok_or_else(|| {
            format!(
                "package.manifest_syntax: `{}` line {line_no}: expected `key = value`",
                manifest_path.display()
            )
        })?;

        match section.as_str() {
            "package" => {
                let key = normalize_key(key)?;
                if key == "authors" {
                    continue;
                }
                if key == "native_libs" {
                    native_libs = parse_string_array_value(value).map_err(|err| {
                        format!(
                            "package.manifest_syntax: `{}` line {line_no}: {err}",
                            manifest_path.display()
                        )
                    })?;
                    continue;
                }
                let value = parse_string_value(value).map_err(|err| {
                    format!(
                        "package.manifest_syntax: `{}` line {line_no}: {err}",
                        manifest_path.display()
                    )
                })?;
                package.insert(key, value);
            }
            "dependencies" => {
                dependencies.push(parse_dependency(key, value).map_err(|err| {
                    format!(
                        "package.manifest_syntax: `{}` line {line_no}: {err}",
                        manifest_path.display()
                    )
                })?);
            }
            "" => {
                return Err(format!(
                    "package.manifest_syntax: `{}` line {line_no}: values must be inside a section",
                    manifest_path.display()
                ));
            }
            other => {
                return Err(format!(
                    "package.manifest_syntax: `{}` line {line_no}: unsupported section `[{}]`",
                    manifest_path.display(),
                    other
                ));
            }
        }
    }

    let name = required_field(&package, "name", &manifest_path)?;
    validate_package_name(&name).map_err(|err| {
        format!(
            "package.name_invalid: `{}` declares invalid package name `{name}`: {err}",
            manifest_path.display()
        )
    })?;

    let version = required_field(&package, "version", &manifest_path)?;
    validate_semver_like(&version).map_err(|err| {
        format!(
            "package.version_invalid: `{}` declares invalid version `{version}`: {err}",
            manifest_path.display()
        )
    })?;

    let entry_raw = required_field(&package, "entry", &manifest_path)?;
    let entry = root.join(&entry_raw);
    if entry.extension().and_then(|ext| ext.to_str()) != Some("orl") {
        return Err(format!(
            "package.entry_invalid: `{}` entry must be a `.orl` file",
            manifest_path.display()
        ));
    }
    if !entry.is_file() {
        return Err(format!(
            "package.entry_missing: `{}` entry `{}` does not exist",
            manifest_path.display(),
            entry.display()
        ));
    }

    let ori_version = required_field(&package, "ori_version", &manifest_path)?;
    let description = package.get("description").cloned();

    Ok(PackageManifest {
        root,
        manifest_path,
        name,
        version,
        entry,
        ori_version,
        description,
        dependencies,
        native_libs,
    })
}

fn parse_dependency(key: &str, value: &str) -> Result<PackageDependency, String> {
    let name = normalize_key(key)?;
    validate_package_name(&name)
        .map_err(|err| format!("dependency name `{name}` is invalid: {err}"))?;
    let value = value.trim();
    if value.starts_with('"') {
        let version = parse_string_value(value)?;
        validate_semver_like(&version)
            .map_err(|err| format!("dependency `{name}` version `{version}` is invalid: {err}"))?;
        return Ok(PackageDependency {
            name,
            requirement: DependencyRequirement::Version(version),
        });
    }
    if value.starts_with('{') {
        let table = parse_inline_table(value)?;
        let path = table
            .get("path")
            .cloned()
            .ok_or_else(|| format!("dependency `{name}` path table requires `path`"))?;
        let version = table.get("version").cloned();
        if let Some(version) = &version {
            validate_semver_like(version).map_err(|err| {
                format!("dependency `{name}` version `{version}` is invalid: {err}")
            })?;
        }
        return Ok(PackageDependency {
            name,
            requirement: DependencyRequirement::Path {
                path: PathBuf::from(path),
                version,
            },
        });
    }
    Err(format!(
        "dependency `{name}` must be a version string or `{{ path = \"...\" }}` table"
    ))
}

fn parse_inline_table(value: &str) -> Result<HashMap<String, String>, String> {
    let trimmed = value.trim();
    if !trimmed.ends_with('}') {
        return Err("inline table must end with `}`".to_string());
    }
    let inner = trimmed
        .strip_prefix('{')
        .ok_or_else(|| "inline table must start with `{`".to_string())?
        .strip_suffix('}')
        .unwrap()
        .trim();
    let mut out = HashMap::new();
    if inner.is_empty() {
        return Ok(out);
    }
    for part in split_top_level(inner, ',') {
        let (key, value) = split_key_value(part.trim())
            .ok_or_else(|| "inline table item must use `key = value`".to_string())?;
        out.insert(normalize_key(key)?, parse_string_value(value)?);
    }
    Ok(out)
}

fn required_field(
    package: &HashMap<String, String>,
    key: &str,
    manifest_path: &Path,
) -> Result<String, String> {
    package.get(key).cloned().ok_or_else(|| {
        format!(
            "package.manifest_missing_field: `{}` requires `[package].{key}`",
            manifest_path.display()
        )
    })
}

fn validate_package_name(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("empty names are not allowed");
    }
    for segment in name.split('.') {
        if segment.is_empty() {
            return Err("empty namespace segments are not allowed");
        }
        let mut chars = segment.chars();
        let first = chars
            .next()
            .ok_or("empty namespace segments are not allowed")?;
        if !(first == '_' || first.is_ascii_alphabetic()) {
            return Err("each segment must start with a letter or `_`");
        }
        if !chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric()) {
            return Err("segments may contain only letters, digits, and `_`");
        }
    }
    Ok(())
}

fn validate_semver_like(version: &str) -> Result<(), &'static str> {
    let mut parts = version.split('.');
    let Some(major) = parts.next() else {
        return Err("expected `major.minor.patch`");
    };
    let Some(minor) = parts.next() else {
        return Err("expected `major.minor.patch`");
    };
    let Some(patch) = parts.next() else {
        return Err("expected `major.minor.patch`");
    };
    if parts.next().is_some() {
        return Err("expected `major.minor.patch`");
    }
    if [major, minor, patch]
        .iter()
        .any(|part| part.is_empty() || !part.chars().all(|ch| ch.is_ascii_digit()))
    {
        return Err("version parts must be decimal numbers");
    }
    Ok(())
}

fn strip_comment(line: &str) -> &str {
    let mut in_string = false;
    let mut escaped = false;
    for (index, ch) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' && in_string {
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_string = !in_string;
            continue;
        }
        if ch == '#' && !in_string {
            return &line[..index];
        }
    }
    line
}

fn split_key_value(line: &str) -> Option<(&str, &str)> {
    let mut in_string = false;
    let mut escaped = false;
    for (index, ch) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' && in_string {
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_string = !in_string;
            continue;
        }
        if ch == '=' && !in_string {
            return Some((&line[..index], &line[index + 1..]));
        }
    }
    None
}

fn split_top_level(value: &str, separator: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut in_string = false;
    let mut escaped = false;
    for (index, ch) in value.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' && in_string {
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_string = !in_string;
            continue;
        }
        if ch == separator && !in_string {
            parts.push(&value[start..index]);
            start = index + ch.len_utf8();
        }
    }
    parts.push(&value[start..]);
    parts
}

fn normalize_key(raw: &str) -> Result<String, String> {
    let key = raw.trim();
    if key.starts_with('"') {
        return parse_string_value(key);
    }
    if key.is_empty() {
        return Err("empty keys are not allowed".to_string());
    }
    Ok(key.to_string())
}

fn parse_string_array_value(raw: &str) -> Result<Vec<String>, String> {
    let value = raw.trim();
    if !value.starts_with('[') || !value.ends_with(']') {
        return Err("expected an array".to_string());
    }
    let inner = value[1..value.len() - 1].trim();
    if inner.is_empty() {
        return Ok(Vec::new());
    }

    let mut items = Vec::new();
    for part in inner.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        items.push(parse_string_value(part)?);
    }
    Ok(items)
}

fn parse_string_value(raw: &str) -> Result<String, String> {
    let value = raw.trim();
    if !value.starts_with('"') || !value.ends_with('"') || value.len() < 2 {
        return Err("expected a quoted string".to_string());
    }
    let inner = &value[1..value.len() - 1];
    let mut out = String::new();
    let mut chars = inner.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        let escaped = chars
            .next()
            .ok_or_else(|| "unterminated string escape".to_string())?;
        match escaped {
            '"' => out.push('"'),
            '\\' => out.push('\\'),
            'n' => out.push('\n'),
            'r' => out.push('\r'),
            't' => out.push('\t'),
            other => {
                return Err(format!("unsupported string escape `\\{other}`"));
            }
        }
    }
    Ok(out)
}

fn copy_package_tree(source: &Path, target: &Path) -> Result<(), String> {
    fs::create_dir_all(target).map_err(|err| {
        format!(
            "package.cache_write_failed: cannot create `{}`: {err}",
            target.display()
        )
    })?;
    copy_dir_recursive(source, target)
}

fn copy_dir_recursive(source: &Path, target: &Path) -> Result<(), String> {
    for entry in fs::read_dir(source).map_err(|err| {
        format!(
            "package.source_read_failed: cannot read `{}`: {err}",
            source.display()
        )
    })? {
        let entry = entry.map_err(|err| {
            format!(
                "package.source_read_failed: cannot read entry in `{}`: {err}",
                source.display()
            )
        })?;
        let file_name = entry.file_name();
        if file_name == ".git" || file_name == "target" {
            continue;
        }
        let source_path = entry.path();
        let target_path = target.join(&file_name);
        let file_type = entry.file_type().map_err(|err| {
            format!(
                "package.source_read_failed: cannot inspect `{}`: {err}",
                source_path.display()
            )
        })?;
        if file_type.is_symlink() {
            return Err(format!(
                "package.symlink_unsupported: `{}` is a symlink",
                source_path.display()
            ));
        }
        if file_type.is_dir() {
            fs::create_dir_all(&target_path).map_err(|err| {
                format!(
                    "package.cache_write_failed: cannot create `{}`: {err}",
                    target_path.display()
                )
            })?;
            copy_dir_recursive(&source_path, &target_path)?;
        } else if file_type.is_file() {
            fs::copy(&source_path, &target_path).map_err(|err| {
                format!(
                    "package.cache_write_failed: cannot copy `{}` to `{}`: {err}",
                    source_path.display(),
                    target_path.display()
                )
            })?;
        }
    }
    Ok(())
}

fn default_package_cache_root() -> Result<PathBuf, String> {
    if let Ok(path) = std::env::var("ORI_PACKAGE_CACHE") {
        if !path.trim().is_empty() {
            return Ok(PathBuf::from(path));
        }
    }
    if let Ok(home) = std::env::var("USERPROFILE") {
        if !home.trim().is_empty() {
            return Ok(PathBuf::from(home).join(".ori").join("packages"));
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        if !home.trim().is_empty() {
            return Ok(PathBuf::from(home).join(".ori").join("packages"));
        }
    }
    Err(
        "package.cache_home_missing: set ORI_PACKAGE_CACHE to choose a local package cache"
            .to_string(),
    )
}
