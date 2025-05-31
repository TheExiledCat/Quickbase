use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let webui_path = Path::new("webui");

    // Run `npm install`
    let status = Command::new("npm")
        .arg("install")
        .current_dir(&webui_path)
        .status()
        .expect("Failed to execute npm install");

    if !status.success() {
        panic!("npm install failed");
    }

    // Run `npm run build`
    let status = Command::new("npm")
        .args(&["run", "build"])
        .current_dir(&webui_path)
        .status()
        .expect("Failed to execute npm run build");

    if !status.success() {
        panic!("npm run build failed");
    }

    // Optionally, copy `webui/dist` to `OUT_DIR` or embed it via include_bytes! etc.
}
