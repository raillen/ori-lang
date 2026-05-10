use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Serialize)]
struct FilePayload {
    path: String,
    text: String,
}

#[derive(Debug, Serialize)]
struct WorkspacePayload {
    root: String,
    files: Vec<String>,
}

#[derive(Debug, Serialize)]
struct CommandOutput {
    command: String,
    status: Option<i32>,
    output: String,
}

#[tauri::command]
fn open_file(path: String) -> Result<FilePayload, String> {
    let path = normalize_path(&path)?;
    if !path.is_file() {
        return Err(format!("not a file: {}", path.display()));
    }
    let text = fs::read_to_string(&path).map_err(|error| error.to_string())?;
    Ok(FilePayload {
        path: path.display().to_string(),
        text,
    })
}

#[tauri::command]
fn save_file(path: String, text: String) -> Result<(), String> {
    let path = normalize_path(&path)?;
    fs::write(&path, text).map_err(|error| error.to_string())
}

#[tauri::command]
fn open_folder(path: String) -> Result<WorkspacePayload, String> {
    let root = normalize_path(&path)?;
    if !root.is_dir() {
        return Err(format!("not a folder: {}", root.display()));
    }
    let mut files = Vec::new();
    collect_zt_files(&root, &root, &mut files).map_err(|error| error.to_string())?;
    files.sort();
    Ok(WorkspacePayload {
        root: root.display().to_string(),
        files,
    })
}

#[tauri::command]
fn run_zt(root: String, command: String) -> Result<CommandOutput, String> {
    let target = normalize_path(&root)?;
    let command_name = safe_command(&command)?;
    let zt = resolve_zt(&target);
    let current_dir = command_dir(&target);
    let output = Command::new(&zt)
        .arg(command_name)
        .arg(&target)
        .current_dir(&current_dir)
        .output()
        .map_err(|error| format!("cannot run {}: {error}", zt.display()))?;
    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    Ok(CommandOutput {
        command: format!("zt {command_name}"),
        status: output.status.code(),
        output: text,
    })
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_file,
            save_file,
            open_folder,
            run_zt
        ])
        .run(tauri::generate_context!())
        .expect("error while running Zenith IDE");
}

fn normalize_path(path: &str) -> Result<PathBuf, String> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err("path is empty".to_string());
    }
    Ok(PathBuf::from(trimmed))
}

fn collect_zt_files(root: &Path, dir: &Path, files: &mut Vec<String>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if should_skip(&name) {
            continue;
        }
        if path.is_dir() {
            collect_zt_files(root, &path, files)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("zt") {
            let relative = path.strip_prefix(root).unwrap_or(&path);
            files.push(relative.display().to_string());
        }
    }
    Ok(())
}

fn should_skip(name: &str) -> bool {
    name.starts_with('.')
        || matches!(
            name,
            "target" | "node_modules" | "__pycache__" | "dist" | "build" | "tmp"
        )
}

fn safe_command(command: &str) -> Result<&'static str, String> {
    match command.trim() {
        "check" => Ok("check"),
        "run" => Ok("run"),
        "build" => Ok("build"),
        "test" => Ok("test"),
        "format" | "fmt" => Ok("fmt"),
        other => Err(format!("unsupported command: {other}")),
    }
}

fn resolve_zt(root: &Path) -> PathBuf {
    let exe = if cfg!(windows) { "zt.exe" } else { "zt" };
    let mut current = if root.is_file() {
        root.parent()
    } else {
        Some(root)
    };
    while let Some(dir) = current {
        let candidate = dir.join(exe);
        if candidate.is_file() {
            return candidate;
        }
        current = dir.parent();
    }
    PathBuf::from(exe)
}

fn command_dir(target: &Path) -> PathBuf {
    if target.is_file() {
        target
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."))
    } else {
        target.to_path_buf()
    }
}
