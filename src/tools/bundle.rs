use std::fs;
use std::path::{Path, PathBuf};

/// Default embedded Apple ICNS icon (alyac-dark) for standalone zero-dependency bundling.
const DEFAULT_APP_ICON_ICNS: &[u8] = include_bytes!("../../assets/brand/icons/alyac-dark.icns");

#[derive(Debug, Clone)]
pub struct BundleOptions {
    pub app_name: String,
    pub bundle_dir: PathBuf,
    pub bundle_id: Option<String>,
    pub bundle_version: Option<String>,
    pub icon_path: Option<String>,
}

impl BundleOptions {
    pub fn new(app_name: &str, output_override: Option<&str>) -> Self {
        let bundle_name = if let Some(out) = output_override {
            if out.ends_with(".app") {
                out.to_string()
            } else {
                format!("{}.app", out)
            }
        } else {
            format!("{}.app", app_name)
        };

        let stem = Path::new(&bundle_name)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(app_name)
            .to_string();

        Self {
            app_name: stem,
            bundle_dir: PathBuf::from(bundle_name),
            bundle_id: None,
            bundle_version: None,
            icon_path: None,
        }
    }

    pub fn binary_path(&self) -> PathBuf {
        self.bundle_dir
            .join("Contents")
            .join("MacOS")
            .join(&self.app_name)
    }

    pub fn create_structure(&self) -> Result<(), String> {
        let macos_dir = self.bundle_dir.join("Contents").join("MacOS");
        let resources_dir = self.bundle_dir.join("Contents").join("Resources");

        fs::create_dir_all(&macos_dir)
            .map_err(|e| format!("Failed to create bundle directory {:?}: {}", macos_dir, e))?;
        fs::create_dir_all(&resources_dir).map_err(|e| {
            format!(
                "Failed to create bundle directory {:?}: {}",
                resources_dir, e
            )
        })?;

        // Write Info.plist
        let info_plist = self.generate_info_plist();
        let plist_path = self.bundle_dir.join("Contents").join("Info.plist");
        fs::write(&plist_path, info_plist)
            .map_err(|e| format!("Failed to write Info.plist at {:?}: {}", plist_path, e))?;

        // Install AppIcon.icns
        self.install_icon()?;

        Ok(())
    }

    pub fn generate_info_plist(&self) -> String {
        let id = self.bundle_id.clone().unwrap_or_else(|| {
            let clean_name = self.app_name.to_lowercase().replace(' ', "-");
            format!("com.alya.{}", clean_name)
        });
        let version = self.bundle_version.as_deref().unwrap_or("1.0.0");

        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>{}</string>
    <key>CFBundleIdentifier</key>
    <string>{}</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>{}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>{}</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>LSMinimumSystemVersion</key>
    <string>11.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
"#,
            self.app_name, id, self.app_name, version
        )
    }

    fn install_icon(&self) -> Result<(), String> {
        let target_icon = self
            .bundle_dir
            .join("Contents")
            .join("Resources")
            .join("AppIcon.icns");

        if let Some(ref custom_icon) = self.icon_path {
            if Path::new(custom_icon).exists() {
                fs::copy(custom_icon, &target_icon)
                    .map_err(|e| format!("Failed to copy icon from '{}': {}", custom_icon, e))?;
                return Ok(());
            } else {
                return Err(format!("Icon file not found: {}", custom_icon));
            }
        }

        // Try local asset file first if available, otherwise write embedded ICNS
        let local_candidates = [
            "assets/brand/icons/alyac-dark.icns",
            "../assets/brand/icons/alyac-dark.icns",
        ];

        for candidate in &local_candidates {
            if Path::new(candidate).exists() {
                if fs::copy(candidate, &target_icon).is_ok() {
                    return Ok(());
                }
            }
        }

        // Write embedded fallback ICNS
        fs::write(&target_icon, DEFAULT_APP_ICON_ICNS)
            .map_err(|e| format!("Failed to write default AppIcon.icns: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bundle_options_and_structure() {
        let temp_dir =
            std::env::temp_dir().join(format!("alya_test_bundle_{}", std::process::id()));
        let bundle_name = temp_dir.join("TestApp.app");
        let opts = BundleOptions::new("TestApp", Some(bundle_name.to_str().unwrap()));

        assert_eq!(opts.app_name, "TestApp");
        assert_eq!(opts.bundle_dir, bundle_name);
        assert_eq!(
            opts.binary_path(),
            bundle_name.join("Contents").join("MacOS").join("TestApp")
        );

        let res = opts.create_structure();
        assert!(res.is_ok(), "create_structure failed: {:?}", res);

        assert!(bundle_name.join("Contents").join("Info.plist").exists());
        assert!(bundle_name
            .join("Contents")
            .join("Resources")
            .join("AppIcon.icns")
            .exists());
        assert!(bundle_name.join("Contents").join("MacOS").exists());

        let plist_content =
            fs::read_to_string(bundle_name.join("Contents").join("Info.plist")).unwrap();
        assert!(plist_content.contains("<string>TestApp</string>"));
        assert!(plist_content.contains("<string>com.alya.testapp</string>"));

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
