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

        let manifest_path = out_dir.join("app.manifest");
        let manifest_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <assemblyIdentity version="1.0.0.0" processorArchitecture="*" name="QRGenerator" type="win32"/>
  <description>QR Generator</description>
  <dependency>
    <dependentAssembly>
      <assemblyIdentity
        type="win32"
        name="Microsoft.Windows.Common-Controls"
        version="6.0.0.0"
        processorArchitecture="*"
        publicKeyToken="6595b64144ccf1df"
        language="*"
      />
    </dependentAssembly>
  </dependency>
  <application xmlns="urn:schemas-microsoft-com:asm.v3">
    <windowsSettings>
      <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true/pm</dpiAware>
      <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2, PerMonitor</dpiAwareness>
    </windowsSettings>
  </application>
</assembly>
"#;
        let _ = std::fs::write(&manifest_path, manifest_xml);

        let rc_content = format!(
            "1 ICON \"{}\"\n1 24 \"{}\"\n",
            icon_path.display().to_string().replace('\\', "\\\\"),
            manifest_path.display().to_string().replace('\\', "\\\\")
        );
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
