fn main() {
    #[cfg(windows)]
    {
        println!("cargo:rerun-if-changed=img/icon.ico");
        println!("cargo:rerun-if-changed=build.rs");

        let out_dir = match std::env::var("OUT_DIR") {
            Ok(dir) => std::path::PathBuf::from(dir),
            Err(_) => return,
        };

        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        let icon_path = std::path::Path::new(&manifest_dir).join("img").join("icon.ico");
        if !icon_path.exists() {
            return;
        }

        let rc_content = format!("1 ICON \"{}\"\n", icon_path.display().to_string().replace('\\', "\\\\"));
        let rc_path = out_dir.join("app.rc");
        let res_path = out_dir.join("app.res");

        if std::fs::write(&rc_path, rc_content).is_ok() {
            let mut candidates = vec![std::path::PathBuf::from("rc.exe")];

            // Buscar en Windows Kits comunes si rc.exe no está en PATH
            if let Ok(entries) = std::fs::read_dir("C:\\Program Files (x86)\\Windows Kits\\10\\bin") {
                for entry in entries.flatten() {
                    let x64_rc = entry.path().join("x64").join("rc.exe");
                    if x64_rc.exists() {
                        candidates.push(x64_rc);
                    }
                }
            }

            for candidate in &candidates {
                let status = std::process::Command::new(candidate)
                    .arg("/fo")
                    .arg(&res_path)
                    .arg(&rc_path)
                    .status();

                if let Ok(s) = status {
                    if s.success() {
                        println!("cargo:rustc-link-arg={}", res_path.display());
                        break;
                    }
                }
            }
        }
    }
}

