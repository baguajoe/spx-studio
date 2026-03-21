mod vst;

#[tauri::command]
fn scan_vst_plugins() -> Vec<vst::VstPlugin> {
    vst::scan_vst_plugins()
}

#[tauri::command]
fn get_vst_paths() -> Vec<String> {
    vst::get_vst_paths()
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            scan_vst_plugins,
            get_vst_paths,
        ])
        .run(tauri::generate_context!())
        .expect("error while running SPX Studio");
}
