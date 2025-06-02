use std::{
    env,
    fs::{self, remove_file},
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let webui_dir = Path::new("webui");

    // 1. Run `npm install`
    let status = Command::new("npm")
        .arg("install")
        .current_dir(webui_dir)
        .status()
        .expect("failed to run `npm install`");
    assert!(status.success());

    // 2. Run `npm run build`
    let status = Command::new("npm")
        .args(&["run", "build"])
        .current_dir(webui_dir)
        .status()
        .expect("failed to run `npm run build`");
    assert!(status.success());

    // 3. Gzip and delete originals
    let dist_path = webui_dir.join("dist");
    compress_and_delete(&dist_path).expect("failed to gzip and clean");

    // 4. Rebuild trigger
    println!("cargo:rerun-if-changed=webui/");
}

fn compress_and_delete(dir: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            compress_and_delete(&path)?;
        } else if path.extension().map_or(true, |e| e != "gz") {
            let data = fs::read(&path)?;
            let gz_path = path.with_extension(format!(
                "{}.gz",
                path.extension().unwrap_or_default().to_string_lossy()
            ));

            let mut encoder = libflate::gzip::Encoder::new(Vec::new())?;
            std::io::copy(&mut data.as_slice(), &mut encoder)?;
            let compressed = encoder.finish().into_result()?;
            fs::write(&gz_path, compressed)?;

            // Delete the original file
            remove_file(&path)?;
        }
    }
    Ok(())
}
