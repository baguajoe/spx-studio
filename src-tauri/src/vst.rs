// =============================================================================
// vst.rs — VST Plugin Host for SPX Studio
// =============================================================================
// Scans for VST2/VST3 plugins on the system and exposes them to the frontend
// =============================================================================

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VstPlugin {
    pub name: String,
    pub path: String,
    pub format: String, // "VST2" or "VST3"
    pub vendor: String,
    pub category: String, // "Instrument", "Effect", "Analyzer"
}

/// Get default VST scan paths per platform
pub fn get_vst_scan_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    #[cfg(target_os = "windows")]
    {
        paths.push(PathBuf::from("C:\\Program Files\\VSTPlugins"));
        paths.push(PathBuf::from("C:\\Program Files\\Steinberg\\VSTPlugins"));
        paths.push(PathBuf::from("C:\\Program Files\\Common Files\\VST3"));
        paths.push(PathBuf::from("C:\\Program Files\\Common Files\\Steinberg\\VST3"));
        if let Ok(appdata) = std::env::var("APPDATA") {
            paths.push(PathBuf::from(format!("{}\\VST3", appdata)));
        }
    }

    #[cfg(target_os = "macos")]
    {
        paths.push(PathBuf::from("/Library/Audio/Plug-Ins/VST"));
        paths.push(PathBuf::from("/Library/Audio/Plug-Ins/VST3"));
        paths.push(PathBuf::from("/Library/Audio/Plug-Ins/Components"));
        if let Some(home) = dirs::home_dir() {
            paths.push(home.join("Library/Audio/Plug-Ins/VST"));
            paths.push(home.join("Library/Audio/Plug-Ins/VST3"));
        }
    }

    #[cfg(target_os = "linux")]
    {
        paths.push(PathBuf::from("/usr/lib/vst"));
        paths.push(PathBuf::from("/usr/lib/vst3"));
        paths.push(PathBuf::from("/usr/local/lib/vst"));
        if let Some(home) = dirs::home_dir() {
            paths.push(home.join(".vst"));
            paths.push(home.join(".vst3"));
        }
    }

    paths
}

/// Scan a directory for VST plugins
pub fn scan_vst_dir(path: &PathBuf) -> Vec<VstPlugin> {
    let mut plugins = Vec::new();

    if !path.exists() { return plugins; }

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            let ext = p.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();

            let format = match ext.as_str() {
                "dll" | "so" => "VST2",
                "vst3" => "VST3",
                "component" => "AU",
                _ => continue,
            };

            let name = p.file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();

            plugins.push(VstPlugin {
                name: name.clone(),
                path: p.to_string_lossy().to_string(),
                format: format.to_string(),
                vendor: "Unknown".to_string(),
                category: guess_category(&name),
            });
        }
    }

    // Recurse into subdirectories
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                plugins.extend(scan_vst_dir(&entry.path()));
            }
        }
    }

    plugins
}

fn guess_category(name: &str) -> String {
    let n = name.to_lowercase();
    if n.contains("synth") || n.contains("piano") || n.contains("organ") ||
       n.contains("bass") || n.contains("drum") || n.contains("sampler") {
        "Instrument".to_string()
    } else if n.contains("eq") || n.contains("comp") || n.contains("reverb") ||
              n.contains("delay") || n.contains("chorus") || n.contains("limiter") {
        "Effect".to_string()
    } else if n.contains("analyzer") || n.contains("meter") || n.contains("scope") {
        "Analyzer".to_string()
    } else {
        "Effect".to_string()
    }
}

/// Tauri command — scan all default VST paths
#[tauri::command]
pub fn scan_vst_plugins() -> Vec<VstPlugin> {
    let paths = get_vst_scan_paths();
    let mut all = Vec::new();
    for path in &paths {
        all.extend(scan_vst_dir(path));
    }
    // Deduplicate by name
    all.sort_by(|a, b| a.name.cmp(&b.name));
    all.dedup_by(|a, b| a.name == b.name);
    all
}

/// Tauri command — scan a custom path
#[tauri::command]
pub fn scan_custom_vst_path(path: String) -> Vec<VstPlugin> {
    scan_vst_dir(&PathBuf::from(path))
}

/// Tauri command — get default scan paths
#[tauri::command]
pub fn get_vst_paths() -> Vec<String> {
    get_vst_scan_paths()
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect()
}
