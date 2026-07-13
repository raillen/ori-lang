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
    /// Remote (or local `file://` / path) Git source. Fetched into the package cache.
    Git {
        url: String,
        /// Preferred pin (exactly one of rev / tag / branch should be set; default branch = `main`).
        rev: Option<String>,
        tag: Option<String>,
        branch: Option<String>,
        /// Optional expected package version from the cloned `ori.pkg.toml`.
        version: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitDependencySpec {
    pub url: String,
    pub rev: Option<String>,
    pub tag: Option<String>,
    pub branch: Option<String>,
    pub version: Option<String>,
}

impl GitDependencySpec {
    pub fn from_requirement(url: String, rev: Option<String>, tag: Option<String>, branch: Option<String>, version: Option<String>) -> Self {
        Self {
            url,
            rev,
            tag,
            branch,
            version,
        }
    }

    pub fn ref_key(&self) -> String {
        if let Some(rev) = &self.rev {
            return format!("rev-{}", sanitize_cache_segment(rev));
        }
        if let Some(tag) = &self.tag {
            return format!("tag-{}", sanitize_cache_segment(tag));
        }
        if let Some(branch) = &self.branch {
            return format!("branch-{}", sanitize_cache_segment(branch));
        }
        "branch-main".to_string()
    }

    pub fn checkout_ref(&self) -> Option<String> {
        self.rev
            .clone()
            .or_else(|| self.tag.clone())
            .or_else(|| self.branch.clone())
    }
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

#[derive(Debug, Clone)]
pub struct GetDependenciesOptions {
    /// Project root, `ori.proj`, `ori.pkg.toml`, or package directory.
    pub path: PathBuf,
    pub cache_root: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDependenciesOutput {
    pub cache_root: PathBuf,
    pub packages: Vec<InstalledPackage>,
}

pub fn run_install_package(options: InstallPackageOptions) -> Result<InstallPackageOutput, String> {
    let cache_root = match options.cache_root {
        Some(path) => path,
        None => default_package_cache_root()?,
    };

    let mut seen = HashSet::new();
    let mut packages = Vec::new();

    // Registry install: `ori install name` or `ori install name@version` (no --path).
    if options.source.is_none()
        && !options.name.starts_with("github.com/")
        && !options.name.starts_with("https://")
        && !options.name.starts_with("http://")
        && !options.name.starts_with("git@")
    {
        let (pkg_name, version_opt) = split_name_version(&options.name);
        let registry = resolve_registry_location(None)?;
        let version = match version_opt {
            Some(v) => v.to_string(),
            None => latest_registry_version(&registry, pkg_name)?,
        };
        let installed_root =
            ensure_version_package_cached(pkg_name, &version, &cache_root, Some(&registry))?;
        install_local_package(
            &installed_root,
            Some(pkg_name),
            &cache_root,
            &mut seen,
            &mut packages,
        )?;
        return Ok(InstallPackageOutput {
            cache_root,
            packages,
        });
    }

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
                    "package.registry_unavailable: cannot fetch `{}`; set ORI_REGISTRY, use `ori install {} --path <dir>`, or a GitHub URL",
                    options.name, options.name
                ));
            }
        }
    };

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
                let root = ensure_version_package_cached(
                    &dependency.name,
                    version,
                    cache_root,
                    resolve_registry_location(None).ok().as_ref(),
                )?;
                install_local_package(
                    &root,
                    Some(&dependency.name),
                    cache_root,
                    seen,
                    packages,
                )?;
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
            DependencyRequirement::Git {
                url,
                rev,
                tag,
                branch,
                version,
            } => {
                let spec = GitDependencySpec::from_requirement(
                    url.clone(),
                    rev.clone(),
                    tag.clone(),
                    branch.clone(),
                    version.clone(),
                );
                let dependency_root =
                    ensure_git_dependency_cached(&dependency.name, &spec, cache_root)?;
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
        let version = table.get("version").cloned();
        if let Some(version) = &version {
            validate_semver_like(version).map_err(|err| {
                format!("dependency `{name}` version `{version}` is invalid: {err}")
            })?;
        }

        if let Some(git) = table.get("git").cloned() {
            let rev = table.get("rev").cloned();
            let tag = table.get("tag").cloned();
            let branch = table.get("branch").cloned();
            let pin_count = [rev.is_some(), tag.is_some(), branch.is_some()]
                .into_iter()
                .filter(|v| *v)
                .count();
            if pin_count > 1 {
                return Err(format!(
                    "dependency `{name}` git table may set only one of `rev`, `tag`, or `branch`"
                ));
            }
            if table.contains_key("path") {
                return Err(format!(
                    "dependency `{name}` cannot combine `git` and `path` in the same table"
                ));
            }
            return Ok(PackageDependency {
                name,
                requirement: DependencyRequirement::Git {
                    url: git,
                    rev,
                    tag,
                    branch,
                    version,
                },
            });
        }

        let path = table
            .get("path")
            .cloned()
            .ok_or_else(|| {
                format!(
                    "dependency `{name}` table requires `path` or `git` (optional `version`/`rev`/`tag`/`branch`)"
                )
            })?;
        return Ok(PackageDependency {
            name,
            requirement: DependencyRequirement::Path {
                path: PathBuf::from(path),
                version,
            },
        });
    }
    Err(format!(
        "dependency `{name}` must be a version string, `{{ path = \"...\" }}`, or `{{ git = \"...\" }}` table"
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

pub fn default_package_cache_root() -> Result<PathBuf, String> {
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

/// Fetch a git dependency into the cache and return the package root (`name/version`).
pub fn ensure_git_dependency_cached(
    expected_name: &str,
    spec: &GitDependencySpec,
    cache_root: &Path,
) -> Result<PathBuf, String> {
    let git_checkout = fetch_git_checkout(spec, cache_root)?;
    let package_root = package_root_from_path(&git_checkout)?;
    let manifest = load_package_manifest(&package_root)?;
    if manifest.name != expected_name {
        return Err(format!(
            "package.dependency_name_mismatch: dependency `{expected_name}` git source declares package `{}`",
            manifest.name
        ));
    }
    if let Some(expected_version) = &spec.version {
        if &manifest.version != expected_version {
            return Err(format!(
                "package.dependency_version_mismatch: dependency `{expected_name}` expected `{expected_version}`, found `{}`",
                manifest.version
            ));
        }
    }

    let target_root = cache_root.join(&manifest.name).join(&manifest.version);
    if target_root.join("ori.pkg.toml").is_file() {
        let cached = load_package_manifest(&target_root)?;
        if cached.name != manifest.name || cached.version != manifest.version {
            return Err(format!(
                "package.cache_conflict: cached package at `{}` does not match `{}` `{}`",
                target_root.display(),
                manifest.name,
                manifest.version
            ));
        }
        return Ok(target_root);
    }

    copy_package_tree(&manifest.root, &target_root)?;
    Ok(target_root)
}

/// Resolve a version-pinned dependency from the local cache, fetching from the
/// configured registry on cache miss when possible.
pub fn resolve_cached_version_package(
    name: &str,
    version: &str,
    cache_root: &Path,
) -> Result<PathBuf, String> {
    ensure_version_package_cached(
        name,
        version,
        cache_root,
        resolve_registry_location(None).ok().as_ref(),
    )
}

/// Ensure `name@version` is materialised under the package cache.
pub fn ensure_version_package_cached(
    name: &str,
    version: &str,
    cache_root: &Path,
    registry: Option<&RegistryLocation>,
) -> Result<PathBuf, String> {
    let root = cache_root.join(name).join(version);
    let manifest_path = root.join("ori.pkg.toml");
    if manifest_path.is_file() {
        let manifest = load_package_manifest(&root)?;
        if manifest.name != name {
            return Err(format!(
                "package.cache_conflict: cache entry `{}` declares name `{}`",
                root.display(),
                manifest.name
            ));
        }
        if manifest.version != version {
            return Err(format!(
                "package.cache_conflict: cache entry `{}` declares version `{}`",
                root.display(),
                manifest.version
            ));
        }
        return Ok(root);
    }

    if let Some(registry) = registry {
        return fetch_package_from_registry(name, version, registry, cache_root);
    }

    Err(format!(
        "package.cache_miss: dependency `{name}` version `{version}` is not in cache at `{}` and no registry is configured (set ORI_REGISTRY, use `path`/`git`, or `ori install --path`)",
        root.display()
    ))
}

/// Fetch all git (and install path) dependencies declared by a project or package manifest.
pub fn run_get_dependencies(options: GetDependenciesOptions) -> Result<GetDependenciesOutput, String> {
    let cache_root = match options.cache_root {
        Some(path) => path,
        None => default_package_cache_root()?,
    };
    let root = package_root_from_path(&options.path)?;
    let mut seen = HashSet::new();
    let mut packages = Vec::new();

    let pkg_manifest = root.join("ori.pkg.toml");
    let proj_manifest = root.join("ori.proj");

    if pkg_manifest.is_file() {
        install_local_package(&root, None, &cache_root, &mut seen, &mut packages)?;
    } else if proj_manifest.is_file() {
        fetch_project_git_dependencies(&proj_manifest, &cache_root, &mut seen, &mut packages)?;
    } else if options.path.is_file() {
        let name = options
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        if name == "ori.pkg.toml" {
            install_local_package(
                &package_root_from_path(&options.path)?,
                None,
                &cache_root,
                &mut seen,
                &mut packages,
            )?;
        } else if name == "ori.proj" {
            fetch_project_git_dependencies(&options.path, &cache_root, &mut seen, &mut packages)?;
        } else {
            return Err(format!(
                "package.get_target_invalid: `{}` is not an Ori project/package root",
                options.path.display()
            ));
        }
    } else {
        return Err(format!(
            "package.get_target_invalid: `{}` has neither `ori.proj` nor `ori.pkg.toml`",
            root.display()
        ));
    }

    Ok(GetDependenciesOutput {
        cache_root,
        packages,
    })
}

fn fetch_project_git_dependencies(
    proj_manifest: &Path,
    cache_root: &Path,
    seen: &mut HashSet<String>,
    packages: &mut Vec<InstalledPackage>,
) -> Result<(), String> {
    let source = fs::read_to_string(proj_manifest).map_err(|err| {
        format!(
            "package.manifest_missing: cannot read `{}`: {err}",
            proj_manifest.display()
        )
    })?;
    let root = proj_manifest
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| {
            format!(
                "package.manifest_invalid: `{}` has no parent",
                proj_manifest.display()
            )
        })?;

    let mut in_dependencies = false;
    for raw_line in source.lines() {
        let line = strip_comment(raw_line).trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            in_dependencies = &line[1..line.len() - 1] == "dependencies";
            continue;
        }
        if !in_dependencies {
            continue;
        }
        let (key, value) = split_key_value(line).ok_or_else(|| {
            format!(
                "package.manifest_syntax: `{}` expected `name = ...` in [dependencies]",
                proj_manifest.display()
            )
        })?;
        let dep = parse_dependency(key, value)?;
        match dep.requirement {
            DependencyRequirement::Git {
                url,
                rev,
                tag,
                branch,
                version,
            } => {
                let spec = GitDependencySpec::from_requirement(url, rev, tag, branch, version);
                let installed_root =
                    ensure_git_dependency_cached(&dep.name, &spec, cache_root)?;
                let manifest = load_package_manifest(&installed_root)?;
                let key = format!("{}@{}", manifest.name, manifest.version);
                if seen.insert(key) {
                    packages.push(InstalledPackage {
                        name: manifest.name,
                        version: manifest.version,
                        source_root: installed_root.clone(),
                        installed_root,
                        already_installed: true,
                    });
                }
            }
            DependencyRequirement::Path { path, version } => {
                let dependency_root = root.join(path);
                if let Some(expected) = &version {
                    let m = load_package_manifest(&dependency_root)?;
                    if m.version != *expected {
                        return Err(format!(
                            "package.dependency_version_mismatch: dependency `{}` expected `{expected}`, found `{}`",
                            dep.name, m.version
                        ));
                    }
                }
                install_local_package(
                    &dependency_root,
                    Some(&dep.name),
                    cache_root,
                    seen,
                    packages,
                )?;
            }
            DependencyRequirement::Version(version) => {
                let root = resolve_cached_version_package(&dep.name, &version, cache_root)?;
                install_local_package(&root, Some(&dep.name), cache_root, seen, packages)?;
            }
        }
    }
    Ok(())
}

fn fetch_git_checkout(spec: &GitDependencySpec, cache_root: &Path) -> Result<PathBuf, String> {
    let url_key = sanitize_cache_segment(&spec.url);
    let ref_key = spec.ref_key();
    let checkout_root = cache_root.join("git").join(url_key).join(ref_key);

    if checkout_root.join("ori.pkg.toml").is_file()
        || checkout_root.join("ori.proj").is_file()
        || has_nested_package_manifest(&checkout_root)
    {
        return Ok(find_package_root_in_checkout(&checkout_root)?);
    }

    if checkout_root.exists() {
        let _ = fs::remove_dir_all(&checkout_root);
    }
    if let Some(parent) = checkout_root.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            format!(
                "package.cache_write_failed: cannot create `{}`: {err}",
                parent.display()
            )
        })?;
    }

    let url = normalize_git_url(&spec.url);
    eprintln!("ori: fetching git dependency {} ({})...", url, spec.ref_key());

    let mut clone = Command::new("git");
    clone.arg("clone");
    if spec.rev.is_none() {
        clone.arg("--depth").arg("1");
        if let Some(tag) = &spec.tag {
            clone.arg("--branch").arg(tag);
        } else if let Some(branch) = &spec.branch {
            clone.arg("--branch").arg(branch);
        } else {
            // default branch when unpinned — try main, fall back to remote default
            clone.arg("--branch").arg("main");
        }
    }
    clone.arg(&url).arg(&checkout_root);

    let status = clone.status().map_err(|err| {
        format!("package.git_clone_failed: failed to invoke git: {err}")
    })?;

    if !status.success() {
        // Retry without --branch main if default failed (remote may use master)
        if spec.rev.is_none() && spec.tag.is_none() && spec.branch.is_none() {
            let _ = fs::remove_dir_all(&checkout_root);
            let status = Command::new("git")
                .arg("clone")
                .arg("--depth")
                .arg("1")
                .arg(&url)
                .arg(&checkout_root)
                .status()
                .map_err(|err| {
                    format!("package.git_clone_failed: failed to invoke git: {err}")
                })?;
            if !status.success() {
                return Err(format!(
                    "package.git_clone_failed: git clone failed for `{url}` (status {status})"
                ));
            }
        } else {
            return Err(format!(
                "package.git_clone_failed: git clone failed for `{url}` (status {status})"
            ));
        }
    }

    if let Some(rev) = &spec.rev {
        let status = Command::new("git")
            .arg("-C")
            .arg(&checkout_root)
            .arg("checkout")
            .arg(rev)
            .status()
            .map_err(|err| format!("package.git_checkout_failed: failed to invoke git: {err}"))?;
        if !status.success() {
            return Err(format!(
                "package.git_checkout_failed: cannot checkout rev `{rev}` in `{}`",
                checkout_root.display()
            ));
        }
    }

    find_package_root_in_checkout(&checkout_root)
}

fn has_nested_package_manifest(root: &Path) -> bool {
    root.join("ori.pkg.toml").is_file() || root.join("ori.proj").is_file()
}

fn find_package_root_in_checkout(checkout_root: &Path) -> Result<PathBuf, String> {
    if checkout_root.join("ori.pkg.toml").is_file() {
        return Ok(checkout_root.to_path_buf());
    }
    // Allow a single top-level subdirectory containing the package (common monorepo layout is out of scope).
    if let Ok(entries) = fs::read_dir(checkout_root) {
        let mut candidates = Vec::new();
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && path.join("ori.pkg.toml").is_file() {
                candidates.push(path);
            }
        }
        if candidates.len() == 1 {
            return Ok(candidates.remove(0));
        }
    }
    Err(format!(
        "package.git_manifest_missing: clone at `{}` has no `ori.pkg.toml` (place it at the repo root)",
        checkout_root.display()
    ))
}

fn normalize_git_url(url: &str) -> String {
    if url.starts_with("github.com/") {
        return format!("https://{url}");
    }
    if url.starts_with("file://") || url.starts_with("http://") || url.starts_with("https://") || url.starts_with("git@") {
        return url.to_string();
    }
    // bare local path
    if Path::new(url).exists() {
        return url.to_string();
    }
    url.to_string()
}

fn sanitize_cache_segment(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '.' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        "x".to_string()
    } else {
        out
    }
}

// ---------------------------------------------------------------------------
// PKG-3 — registry (filesystem + optional HTTP tarball)
// ---------------------------------------------------------------------------

/// Where packages are published to / fetched from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryLocation {
    /// Directory layout: `{root}/packages/{name}/{version}/…` + `versions.json`.
    Path(PathBuf),
    /// HTTP(S) base URL (no trailing slash). Fetch uses
    /// `{base}/packages/{name}/{version}.tar.gz`; publish uses HTTP PUT of the same.
    Http(String),
}

#[derive(Debug, Clone)]
pub struct PublishPackageOptions {
    pub path: PathBuf,
    /// Override `ORI_REGISTRY`.
    pub registry: Option<String>,
    /// Override `ORI_REGISTRY_TOKEN` (HTTP publish Authorization Bearer).
    pub token: Option<String>,
    /// Replace an existing `name@version` in the registry.
    pub force: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishPackageOutput {
    pub name: String,
    pub version: String,
    pub registry: String,
    pub location: String,
}

/// Publish a validated package into the configured registry.
pub fn run_publish_package(options: PublishPackageOptions) -> Result<PublishPackageOutput, String> {
    let root = package_root_from_path(&options.path)?;
    let manifest = load_package_manifest(&root)?;
    // Reject path-only publish of packages that cannot stand alone? Allow all.

    let registry = resolve_registry_location(options.registry.as_deref())?;
    let token = options
        .token
        .or_else(|| std::env::var("ORI_REGISTRY_TOKEN").ok())
        .filter(|t| !t.trim().is_empty());

    match &registry {
        RegistryLocation::Path(reg_root) => {
            let dest = reg_root
                .join("packages")
                .join(&manifest.name)
                .join(&manifest.version);
            if dest.join("ori.pkg.toml").is_file() && !options.force {
                return Err(format!(
                    "package.publish_exists: `{}@{}` already published at `{}` (pass --force to replace)",
                    manifest.name,
                    manifest.version,
                    dest.display()
                ));
            }
            if dest.exists() {
                fs::remove_dir_all(&dest).map_err(|err| {
                    format!(
                        "package.publish_failed: cannot replace `{}`: {err}",
                        dest.display()
                    )
                })?;
            }
            copy_package_tree(&manifest.root, &dest)?;
            update_versions_index(reg_root, &manifest.name, &manifest.version)?;
            update_global_index(reg_root, &manifest.name, &manifest.version)?;

            // Also keep a downloadable tarball next to the tree (for HTTP mirrors).
            let tarball = reg_root
                .join("packages")
                .join(&manifest.name)
                .join(format!("{}.tar.gz", manifest.version));
            create_package_tarball(&manifest.root, &tarball)?;

            Ok(PublishPackageOutput {
                name: manifest.name,
                version: manifest.version,
                registry: reg_root.display().to_string(),
                location: dest.display().to_string(),
            })
        }
        RegistryLocation::Http(base) => {
            let tarball_url = format!(
                "{}/packages/{}/{}.tar.gz",
                base.trim_end_matches('/'),
                manifest.name,
                manifest.version
            );
            let tmp = std::env::temp_dir().join(format!(
                "ori_publish_{}_{}.tar.gz",
                sanitize_cache_segment(&manifest.name),
                manifest.version
            ));
            create_package_tarball(&manifest.root, &tmp)?;
            http_put_file(&tarball_url, &tmp, token.as_deref(), options.force)?;
            let _ = fs::remove_file(&tmp);

            // Best-effort: upload versions.json merge is not possible without GET+PUT race;
            // clients can list via the tarball convention. Document that file registries
            // own the index; HTTP registries may only host tarballs.
            Ok(PublishPackageOutput {
                name: manifest.name,
                version: manifest.version,
                registry: base.clone(),
                location: tarball_url,
            })
        }
    }
}

pub fn resolve_registry_location(override_url: Option<&str>) -> Result<RegistryLocation, String> {
    let raw = override_url
        .map(|s| s.to_string())
        .or_else(|| std::env::var("ORI_REGISTRY").ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            "package.registry_unconfigured: set ORI_REGISTRY to a directory path or http(s) base URL (e.g. /var/ori-registry or https://registry.example/ori)"
                .to_string()
        })?;

    if raw.starts_with("http://") || raw.starts_with("https://") {
        return Ok(RegistryLocation::Http(raw.trim_end_matches('/').to_string()));
    }
    let path = if let Some(rest) = raw.strip_prefix("file://") {
        PathBuf::from(rest)
    } else {
        PathBuf::from(&raw)
    };
    Ok(RegistryLocation::Path(path))
}

fn fetch_package_from_registry(
    name: &str,
    version: &str,
    registry: &RegistryLocation,
    cache_root: &Path,
) -> Result<PathBuf, String> {
    let target = cache_root.join(name).join(version);
    match registry {
        RegistryLocation::Path(reg_root) => {
            let source = reg_root.join("packages").join(name).join(version);
            if !source.join("ori.pkg.toml").is_file() {
                return Err(format!(
                    "package.registry_miss: `{name}@{version}` not found in registry at `{}`",
                    source.display()
                ));
            }
            let manifest = load_package_manifest(&source)?;
            if manifest.name != name || manifest.version != version {
                return Err(format!(
                    "package.registry_conflict: registry entry `{}` declares `{}@{}`",
                    source.display(),
                    manifest.name,
                    manifest.version
                ));
            }
            eprintln!(
                "ori: fetching `{name}@{version}` from registry {}...",
                reg_root.display()
            );
            copy_package_tree(&source, &target)?;
            Ok(target)
        }
        RegistryLocation::Http(base) => {
            let url = format!(
                "{}/packages/{}/{}.tar.gz",
                base.trim_end_matches('/'),
                name,
                version
            );
            eprintln!("ori: fetching `{name}@{version}` from {url}...");
            let tmp_tar = std::env::temp_dir().join(format!(
                "ori_fetch_{}_{}.tar.gz",
                sanitize_cache_segment(name),
                version
            ));
            http_get_file(&url, &tmp_tar)?;
            let tmp_extract = std::env::temp_dir().join(format!(
                "ori_fetch_{}_{}_dir",
                sanitize_cache_segment(name),
                version
            ));
            let _ = fs::remove_dir_all(&tmp_extract);
            fs::create_dir_all(&tmp_extract).map_err(|err| {
                format!(
                    "package.cache_write_failed: cannot create `{}`: {err}",
                    tmp_extract.display()
                )
            })?;
            extract_package_tarball(&tmp_tar, &tmp_extract)?;
            let _ = fs::remove_file(&tmp_tar);
            let package_root = find_package_root_in_checkout(&tmp_extract).or_else(|_| {
                // tarball may contain files at top level
                if tmp_extract.join("ori.pkg.toml").is_file() {
                    Ok(tmp_extract.clone())
                } else {
                    Err(format!(
                        "package.registry_invalid: tarball for `{name}@{version}` has no ori.pkg.toml"
                    ))
                }
            })?;
            let manifest = load_package_manifest(&package_root)?;
            if manifest.name != name || manifest.version != version {
                return Err(format!(
                    "package.registry_conflict: tarball declares `{}@{}`",
                    manifest.name, manifest.version
                ));
            }
            copy_package_tree(&package_root, &target)?;
            let _ = fs::remove_dir_all(&tmp_extract);
            Ok(target)
        }
    }
}

fn latest_registry_version(registry: &RegistryLocation, name: &str) -> Result<String, String> {
    match registry {
        RegistryLocation::Path(reg_root) => {
            let versions_path = reg_root.join("packages").join(name).join("versions.json");
            if versions_path.is_file() {
                let text = fs::read_to_string(&versions_path).map_err(|err| {
                    format!(
                        "package.registry_read_failed: cannot read `{}`: {err}",
                        versions_path.display()
                    )
                })?;
                let versions = parse_versions_json(&text)?;
                return versions.last().cloned().ok_or_else(|| {
                    format!("package.registry_miss: package `{name}` has no versions in registry")
                });
            }
            // Fall back: scan version directories
            let dir = reg_root.join("packages").join(name);
            let mut versions = list_version_dirs(&dir)?;
            versions.sort_by(|a, b| compare_semver_like(a, b));
            versions.last().cloned().ok_or_else(|| {
                format!(
                    "package.registry_miss: package `{name}` not found under `{}`",
                    dir.display()
                )
            })
        }
        RegistryLocation::Http(base) => {
            let url = format!(
                "{}/packages/{}/versions.json",
                base.trim_end_matches('/'),
                name
            );
            let tmp = std::env::temp_dir().join(format!(
                "ori_versions_{}.json",
                sanitize_cache_segment(name)
            ));
            http_get_file(&url, &tmp).map_err(|err| {
                format!(
                    "package.registry_miss: cannot read versions for `{name}` from `{url}`: {err}"
                )
            })?;
            let text = fs::read_to_string(&tmp).map_err(|err| {
                format!("package.registry_read_failed: cannot read versions file: {err}")
            })?;
            let _ = fs::remove_file(&tmp);
            let versions = parse_versions_json(&text)?;
            versions.last().cloned().ok_or_else(|| {
                format!("package.registry_miss: package `{name}` has empty versions.json")
            })
        }
    }
}

fn update_versions_index(reg_root: &Path, name: &str, version: &str) -> Result<(), String> {
    let dir = reg_root.join("packages").join(name);
    fs::create_dir_all(&dir).map_err(|err| {
        format!(
            "package.publish_failed: cannot create `{}`: {err}",
            dir.display()
        )
    })?;
    let path = dir.join("versions.json");
    let mut versions = if path.is_file() {
        let text = fs::read_to_string(&path).unwrap_or_else(|_| "{\"versions\":[]}".into());
        parse_versions_json(&text).unwrap_or_default()
    } else {
        Vec::new()
    };
    if !versions.iter().any(|v| v == version) {
        versions.push(version.to_string());
    }
    versions.sort_by(|a, b| compare_semver_like(a, b));
    let body = format!(
        "{{\n  \"versions\": [{}]\n}}\n",
        versions
            .iter()
            .map(|v| format!("\"{v}\""))
            .collect::<Vec<_>>()
            .join(", ")
    );
    fs::write(&path, body).map_err(|err| {
        format!(
            "package.publish_failed: cannot write `{}`: {err}",
            path.display()
        )
    })
}

fn update_global_index(reg_root: &Path, name: &str, version: &str) -> Result<(), String> {
    let path = reg_root.join("index.json");
    let mut packages: HashMap<String, Vec<String>> = HashMap::new();
    if path.is_file() {
        if let Ok(text) = fs::read_to_string(&path) {
            packages = parse_global_index(&text).unwrap_or_default();
        }
    }
    let entry = packages.entry(name.to_string()).or_default();
    if !entry.iter().any(|v| v == version) {
        entry.push(version.to_string());
    }
    entry.sort_by(|a, b| compare_semver_like(a, b));

    let mut lines = vec!["{".to_string(), "  \"packages\": {".to_string()];
    let mut names: Vec<_> = packages.keys().cloned().collect();
    names.sort();
    for (i, n) in names.iter().enumerate() {
        let vers = &packages[n];
        let list = vers
            .iter()
            .map(|v| format!("\"{v}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let comma = if i + 1 < names.len() { "," } else { "" };
        lines.push(format!("    \"{n}\": [{list}]{comma}"));
    }
    lines.push("  }".to_string());
    lines.push("}".to_string());
    lines.push(String::new());
    fs::write(&path, lines.join("\n")).map_err(|err| {
        format!(
            "package.publish_failed: cannot write `{}`: {err}",
            path.display()
        )
    })
}

fn parse_versions_json(text: &str) -> Result<Vec<String>, String> {
    // Minimal JSON: {"versions":["0.1.0","0.2.0"]}
    let trimmed = text.trim();
    let Some(start) = trimmed.find('[') else {
        return Err("package.registry_index_invalid: versions.json missing array".into());
    };
    let Some(end) = trimmed.rfind(']') else {
        return Err("package.registry_index_invalid: versions.json missing array end".into());
    };
    let inner = &trimmed[start + 1..end];
    let mut out = Vec::new();
    for part in inner.split(',') {
        let p = part.trim().trim_matches('"').trim();
        if !p.is_empty() {
            out.push(p.to_string());
        }
    }
    Ok(out)
}

fn parse_global_index(text: &str) -> Result<HashMap<String, Vec<String>>, String> {
    // Very small parser for {"packages":{"a":["0.1.0"],"b":["1.0.0"]}}
    let mut map = HashMap::new();
    let Some(packages_pos) = text.find("\"packages\"") else {
        return Ok(map);
    };
    let rest = &text[packages_pos..];
    let Some(brace) = rest.find('{') else {
        return Ok(map);
    };
    let mut depth = 0i32;
    let mut body_start = None;
    let mut body_end = None;
    for (i, ch) in rest[brace..].char_indices() {
        match ch {
            '{' => {
                if depth == 0 {
                    body_start = Some(brace + i + 1);
                }
                depth += 1;
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    body_end = Some(brace + i);
                    break;
                }
            }
            _ => {}
        }
    }
    let (Some(s), Some(e)) = (body_start, body_end) else {
        return Ok(map);
    };
    let body = &rest[s..e];
    // Split top-level "name": [...]
    let mut i = 0;
    let bytes = body.as_bytes();
    while i < bytes.len() {
        while i < bytes.len()
            && ((bytes[i] as char).is_whitespace() || bytes[i] == b',')
        {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        if bytes[i] != b'"' {
            i += 1;
            continue;
        }
        i += 1;
        let name_start = i;
        while i < bytes.len() && bytes[i] != b'"' {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        let name = &body[name_start..i];
        i += 1;
        while i < bytes.len() && bytes[i] != b'[' {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        let arr_start = i;
        while i < bytes.len() && bytes[i] != b']' {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        let arr = &body[arr_start..=i];
        let versions = parse_versions_json(&format!("{{\"versions\":{arr}}}")).unwrap_or_default();
        map.insert(name.to_string(), versions);
        i += 1;
    }
    Ok(map)
}

fn list_version_dirs(dir: &Path) -> Result<Vec<String>, String> {
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in fs::read_dir(dir).map_err(|err| {
        format!(
            "package.registry_read_failed: cannot read `{}`: {err}",
            dir.display()
        )
    })? {
        let entry = entry.map_err(|err| format!("package.registry_read_failed: {err}"))?;
        let path = entry.path();
        if path.is_dir() && path.join("ori.pkg.toml").is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                out.push(name.to_string());
            }
        }
    }
    Ok(out)
}

fn compare_semver_like(a: &str, b: &str) -> std::cmp::Ordering {
    let parse = |s: &str| -> (u64, u64, u64) {
        let mut parts = s.split('.');
        let maj = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
        let min = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
        let pat = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
        (maj, min, pat)
    };
    parse(a).cmp(&parse(b))
}

fn split_name_version(spec: &str) -> (&str, Option<&str>) {
    if let Some((name, ver)) = spec.rsplit_once('@') {
        if !name.is_empty() && !ver.is_empty() && ver.chars().next().is_some_and(|c| c.is_ascii_digit())
        {
            return (name, Some(ver));
        }
    }
    (spec, None)
}

fn create_package_tarball(source_root: &Path, tarball: &Path) -> Result<(), String> {
    if let Some(parent) = tarball.parent() {
        fs::create_dir_all(parent).map_err(|err| {
            format!(
                "package.publish_failed: cannot create `{}`: {err}",
                parent.display()
            )
        })?;
    }
    // tar -czf tarball -C source_root .
    let status = Command::new("tar")
        .arg("-czf")
        .arg(tarball)
        .arg("-C")
        .arg(source_root)
        .arg(".")
        .status()
        .map_err(|err| format!("package.tarball_failed: failed to invoke tar: {err}"))?;
    if !status.success() {
        return Err(format!(
            "package.tarball_failed: tar exited with status {status}"
        ));
    }
    Ok(())
}

fn extract_package_tarball(tarball: &Path, dest: &Path) -> Result<(), String> {
    let status = Command::new("tar")
        .arg("-xzf")
        .arg(tarball)
        .arg("-C")
        .arg(dest)
        .status()
        .map_err(|err| format!("package.tarball_failed: failed to invoke tar: {err}"))?;
    if !status.success() {
        return Err(format!(
            "package.tarball_failed: tar extract exited with status {status}"
        ));
    }
    Ok(())
}

fn http_get_file(url: &str, dest: &Path) -> Result<(), String> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).ok();
    }
    let status = Command::new("curl")
        .arg("-fsSL")
        .arg("-o")
        .arg(dest)
        .arg(url)
        .status()
        .map_err(|err| format!("package.http_failed: failed to invoke curl: {err}"))?;
    if !status.success() {
        return Err(format!(
            "package.http_failed: GET `{url}` failed (curl status {status})"
        ));
    }
    Ok(())
}

fn http_put_file(url: &str, file: &Path, token: Option<&str>, force: bool) -> Result<(), String> {
    // If not forcing, refuse overwrite when resource exists (HEAD).
    if !force {
        let head = Command::new("curl")
            .arg("-sS")
            .arg("-o")
            .arg("/dev/null")
            .arg("-w")
            .arg("%{http_code}")
            .arg("-I")
            .arg(url)
            .output();
        if let Ok(out) = head {
            let code = String::from_utf8_lossy(&out.stdout);
            if code.trim() == "200" {
                return Err(format!(
                    "package.publish_exists: `{url}` already exists (pass --force to replace)"
                ));
            }
        }
    }
    let mut cmd = Command::new("curl");
    cmd.arg("-fsSL").arg("-X").arg("PUT").arg("-T").arg(file);
    if let Some(token) = token {
        cmd.arg("-H").arg(format!("Authorization: Bearer {token}"));
    }
    cmd.arg(url);
    let status = cmd
        .status()
        .map_err(|err| format!("package.http_failed: failed to invoke curl: {err}"))?;
    if !status.success() {
        return Err(format!(
            "package.http_failed: PUT `{url}` failed (curl status {status}); file registries do not need HTTP — set ORI_REGISTRY to a directory path"
        ));
    }
    Ok(())
}
